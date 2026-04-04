use super::{Header, OutboundHeader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailureHeader {
    /// 400 URI Failure - URI has failed to acquire
    ///
    /// Indicates a fatal `URI` failure. The `URI` is not retrievable from this
    /// source. As with [`Self::UriDone`], [`Self::UriStart`] is not required to
    /// precede this message
    ///
    /// Fields:
    /// - URI
    /// - Message
    UriFailure,

    /// 401 General Failure - Method did not like something sent to it
    ///
    /// Indicates that some unspecific failure has occurred and the method is unable
    /// to continue. The method should terminate after sending this message. It
    /// is intended to check for invalid configuration options or other severe conditions.
    ///
    /// Fields:
    /// - Message
    GeneralFailure,

    /// 402 Authorization Required - Method requires authorization to access the
    /// URI. Authorization is User/Pass
    ///
    /// The method requires a `Username` and `Password` pair to continue. After
    /// sending this message the method will expect APT to send a [`Self::AuthorizationCredentials`] message with the credentials. The `Site` field
    /// message with the required information. It is possible for a method to send
    /// this multiple times.
    ///
    /// Fields:
    /// - Site
    AuthorizationRequired,

    /// 403 Media Failure - Method requires a media change
    ///
    /// A method that deals with multiple media requires that a new media be inserted.
    /// The `Media` field contains the name of the media to be inserted.
    ///
    /// Fields:
    /// - Media
    /// - Drive
    MediaFailure,
}

impl Header for FailureHeader {
    fn code(&self) -> u16 {
        match self {
            Self::UriFailure => 400,
            Self::GeneralFailure => 401,
            Self::AuthorizationRequired => 402,
            Self::MediaFailure => 403,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::UriFailure => "URI Failure",
            Self::GeneralFailure => "General Failure",
            Self::AuthorizationRequired => "Authorization Required",
            Self::MediaFailure => "Media Failure",
        }
    }
}

impl OutboundHeader for FailureHeader {}
