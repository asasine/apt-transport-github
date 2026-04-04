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
        RefMessage { header: &message.header, fields: &message.fields }
    }
}
