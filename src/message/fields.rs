use core::{borrow::Borrow, str::FromStr};

/// The fields of a message are key-value pairs that come after the header.
///
/// The valid keys for a message depends on its header.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Fields(Vec<(String, String)>);
impl Fields {
    pub fn iter(&self) -> impl Iterator<Item = &(String, String)> {
        self.0.iter()
    }

    /// Find the value of a field by its key.
    ///
    /// This is short-circuiting: if there are multiple fields with the same key,
    /// only the first one will be returned. If all fields with the same key are
    /// needed, use [`Self::find_all`].
    pub fn find(&self, key: impl Borrow<str>) -> Option<&str> {
        let key = key.borrow();
        self.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    /// Find all values of fields with the same key.
    ///
    /// If only one field with the same key is expected, use [`Self::find`] instead.
    pub fn find_all(&self, key: impl Borrow<str>) -> impl Iterator<Item = &str> {
        self.iter().filter_map(move |(k, v)| {
            if k == key.borrow() {
                Some(v.as_str())
            } else {
                None
            }
        })
    }
}

impl<'a> FromIterator<&'a str> for Fields {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let fields = iter.into_iter().filter_map(|line| line.split_once(": "));

        Self::from_iter(fields)
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for Fields {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        let fields = iter
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));

        Self::from_iter(fields)
    }
}

impl FromIterator<(String, String)> for Fields {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Fields {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Fields {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod find {
        use super::*;

        #[test]
        fn single_field() {
            let fields: Fields = vec![("Key", "Value")].into_iter().collect();
            assert_eq!(fields.find("Key"), Some("Value"));
        }

        #[test]
        fn multiple_fields() {
            let fields: Fields = vec![("Key", "Value1"), ("Key", "Value2")]
                .into_iter()
                .collect();
            assert_eq!(fields.find("Key"), Some("Value1"));
        }

        #[test]
        fn missing_field() {
            let fields: Fields = vec![("Key", "Value")].into_iter().collect();
            assert_eq!(fields.find("Missing"), None);
        }
    }

    mod find_all {
        use super::*;

        #[test]
        fn single_field() {
            let fields: Fields = vec![("Key", "Value")].into_iter().collect();
            let values: Vec<&str> = fields.find_all("Key").collect();
            assert_eq!(values, vec!["Value"]);
        }

        #[test]
        fn multiple_fields() {
            let fields: Fields = vec![("Key", "Value1"), ("Key", "Value2")]
                .into_iter()
                .collect();
            let values: Vec<&str> = fields.find_all("Key").collect();
            assert_eq!(values, vec!["Value1", "Value2"]);
        }

        #[test]
        fn missing_field() {
            let fields: Fields = vec![("Key", "Value")].into_iter().collect();
            let values: Vec<&str> = fields.find_all("Missing").collect();
            assert!(values.is_empty());
        }
    }
}
