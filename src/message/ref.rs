use core::fmt::Display;
use super::{Fields, Message, OutboundHeader};

/// A reference to a message, which can be used for printing without consuming the message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefMessage<'a, H> {
    pub header: &'a H,
    pub fields: &'a Fields,
}

impl<H: OutboundHeader> Display for RefMessage<'_, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.header.display())?;
        for (key, value) in self.fields.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }

        Ok(())
    }
}

impl<'a, H> From<&'a Message<H>> for RefMessage<'a, H> {
    fn from(message: &'a Message<H>) -> Self {
        RefMessage {
            header: &message.header,
            fields: &message.fields,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{InformationalHeader, FailureHeader};

    mod display {
        use super::*;

        #[test]
        fn no_fields() {
            let header = InformationalHeader::Capabilities;
            let fields = Fields::default();
            let msg = RefMessage { header: &header, fields: &fields };
            assert_eq!(msg.to_string(), "100 Capabilities\n");
        }

        #[test]
        fn single_field() {
            let header = InformationalHeader::Log;
            let fields: Fields = vec![("Message", "hello")].into_iter().collect();
            let msg = RefMessage { header: &header, fields: &fields };
            assert_eq!(msg.to_string(), "101 Log\nMessage: hello\n");
        }

        #[test]
        fn multiple_fields() {
            let header = FailureHeader::UriFailure;
            let fields: Fields = vec![("URI", "http://example.com"), ("Message", "not found")]
                .into_iter()
                .collect();
            let msg = RefMessage { header: &header, fields: &fields };
            assert_eq!(
                msg.to_string(),
                "400 URI Failure\nURI: http://example.com\nMessage: not found\n"
            );
        }
    }

    #[test]
    fn from() {
        let msg = Message {
            header: InformationalHeader::Status,
            fields: vec![("Message", "downloading")].into_iter().collect(),
        };
        let ref_msg = RefMessage::from(&msg);
        assert_eq!(ref_msg.header, &msg.header);
        assert_eq!(ref_msg.fields, &msg.fields);
    }
}
