use super::{Fields, InboundHeader, OutboundHeader, r#ref::RefMessage};
use core::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Message<H> {
    pub header: H,
    pub fields: Fields,
}

impl<H: InboundHeader + FromStr> FromStr for Message<H> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().ok_or(())?.parse().map_err(|_| ())?;
        let fields = lines.collect();
        Ok(Self { header, fields })
    }
}

impl<H: OutboundHeader> Display for Message<H> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&RefMessage::from(self), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{FailureHeader, InformationalHeader, RequestHeader};

    mod display {
        use super::*;

        #[test]
        fn no_fields() {
            let msg = Message {
                header: InformationalHeader::Capabilities,
                fields: Fields::default(),
            };
            assert_eq!(msg.to_string(), "100 Capabilities\n");
        }

        #[test]
        fn single_field() {
            let msg = Message {
                header: InformationalHeader::Log,
                fields: vec![("Message", "hello")].into_iter().collect(),
            };
            assert_eq!(msg.to_string(), "101 Log\nMessage: hello\n");
        }

        #[test]
        fn multiple_fields() {
            let msg = Message {
                header: FailureHeader::UriFailure,
                fields: vec![("URI", "http://example.com"), ("Message", "not found")]
                    .into_iter()
                    .collect(),
            };
            assert_eq!(
                msg.to_string(),
                "400 URI Failure\nURI: http://example.com\nMessage: not found\n"
            );
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn header_and_fields() {
            let msg: Message<RequestHeader> = r#"600 URI Acquire
URI: http://example.com
Filename: /tmp/file"#
                .parse()
                .unwrap();
            assert_eq!(msg.header, RequestHeader::UriAcquire);
            assert_eq!(msg.fields.find("URI"), Some("http://example.com"));
            assert_eq!(msg.fields.find("Filename"), Some("/tmp/file"));
        }

        #[test]
        fn header_only() {
            let msg: Message<RequestHeader> = "601 Configuration".parse().unwrap();
            assert_eq!(msg.header, RequestHeader::Configuration);
            assert_eq!(msg.fields.find("Config-Item"), None);
        }

        #[test]
        fn invalid_header() {
            assert_eq!("999 Invalid".parse::<Message<RequestHeader>>(), Err(()));
        }

        #[test]
        fn empty_string() {
            assert_eq!("".parse::<Message<RequestHeader>>(), Err(()));
        }
    }
}
