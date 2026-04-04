use core::str::FromStr;

use super::{Header, InboundHeader, OutboundHeader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestHeader {
    /// 600 URI Acquire - Request a URI be acquired
    ///
    /// APT is requesting that a new `URI` be added to the acquire list. `Last-Modified`
    /// has the time stamp of the currently cache file if applicable. `Filename` is
    /// the name of the file that the acquired `URI` should be written to.
    ///
    /// Fields:
    /// - URI
    /// - Filename
    /// - Last-Modified
    UriAcquire,

    /// 601 Configuration - Sends the configuration space
    ///
    /// APT is sending the configuration space to the method. A series of `Config-Item`
    /// fields will be part of this message, each containing an entry from the configuration
    /// space.
    ///
    /// Fields:
    /// - Config-Item
    Configuration,

    /// 602 Authorization Credentials - Response to [`Self::AuthorizationRequired`].
    ///
    /// This is sent in response to a [`Self::AuthorizationRequired`] message. It
    /// contains the entered username and password.
    ///
    /// Fields:
    /// - Site
    /// - User
    /// - Password
    AuthorizationCredentials,

    /// 603 Media Changed - Response to [`Self::MediaFailure`].
    ///
    /// This is sent in response to a [`Self::MediaFailure`] message. It indicates
    /// that the user has changed media and it is safe to proceed.
    ///
    /// Fields:
    /// - Media
    /// - Fail
    MediaChanged,
}

impl Header for RequestHeader {
    fn code(&self) -> u16 {
        match self {
            Self::UriAcquire => 600,
            Self::Configuration => 601,
            Self::AuthorizationCredentials => 602,
            Self::MediaChanged => 603,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::UriAcquire => "URI Acquire",
            Self::Configuration => "Configuration",
            Self::AuthorizationCredentials => "Authorization Credentials",
            Self::MediaChanged => "Media Changed",
        }
    }
}

impl FromStr for RequestHeader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "600 URI Acquire" => Ok(Self::UriAcquire),
            "601 Configuration" => Ok(Self::Configuration),
            "602 Authorization Credentials" => Ok(Self::AuthorizationCredentials),
            "603 Media Changed" => Ok(Self::MediaChanged),
            _ => Err(()),
        }
    }
}

impl InboundHeader for RequestHeader {}

// Even though these are technically inbound headers, implementing this trait allows
// us to print them using the same code as outbound headers.
impl OutboundHeader for RequestHeader {}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_str {
        use super::*;

        #[test]
        fn code_description() {
            assert_eq!("600 URI Acquire".parse(), Ok(RequestHeader::UriAcquire));
            assert_eq!(
                "601 Configuration".parse(),
                Ok(RequestHeader::Configuration)
            );
            assert_eq!(
                "602 Authorization Credentials".parse(),
                Ok(RequestHeader::AuthorizationCredentials)
            );
            assert_eq!("603 Media Changed".parse(), Ok(RequestHeader::MediaChanged));
        }

        #[test]
        fn invalid() {
            assert_eq!("999 Invalid".parse::<RequestHeader>(), Err(()));
        }
    }
}
