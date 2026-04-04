use core::{fmt::Display, str::FromStr};
use super::{Fields, InboundHeader, OutboundHeader, r#ref::RefMessage};


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
