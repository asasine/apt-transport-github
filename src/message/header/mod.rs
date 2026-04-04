mod request;
mod failure;
mod success;
mod informational;

pub use request::RequestHeader;
pub use failure::FailureHeader;
pub use success::SuccessHeader;
pub use informational::InformationalHeader;

use core::{fmt::Display, str::FromStr};


pub trait Header {
    fn code(&self) -> u16;
    fn description(&self) -> &'static str;

    /// Returns a displayable version of the header, which is the code followed by the description.
    fn display(&self) -> impl Display {
        struct DisplayHeader(u16, &'static str);
        impl Display for DisplayHeader {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} {}", self.0, self.1)
            }
        }

        DisplayHeader(self.code(), self.description())
    }
}

pub trait InboundHeader: Header + FromStr {}
pub trait OutboundHeader: Header {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        struct TestHeader;
        impl Header for TestHeader {
            fn code(&self) -> u16 {
                123
            }

            fn description(&self) -> &'static str {
                "Test Header"
            }
        }

        let header = TestHeader;
        assert_eq!(header.display().to_string(), "123 Test Header");
    }
}
