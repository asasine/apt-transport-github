//! This module contains factory functions for creating messages with specific headers
//! and fields.

use crate::message::{FailureHeader, FailureMessage, InformationalHeader, InformationalMessage};

/// Create a Capabilities message with no fields.
pub fn capabilities() -> InformationalMessage {
    InformationalMessage {
        header: InformationalHeader::Capabilities,
        fields: Default::default(),
    }
}

/// Create a Log message with the given `message`.
pub fn log(message: impl Into<String>) -> InformationalMessage {
    InformationalMessage {
        header: InformationalHeader::Log,
        fields: [("Message".to_string(), message.into())]
            .into_iter()
            .collect(),
    }
}

/// Create a Status message with the given `message`.
pub fn status(message: impl Into<String>) -> InformationalMessage {
    InformationalMessage {
        header: InformationalHeader::Status,
        fields: [("Message".to_string(), message.into())]
            .into_iter()
            .collect(),
    }
}

/// Create a URI Failure message indicating a URI failure with the given `uri` and `message`.
pub fn uri_failure(uri: impl Into<String>, message: impl Into<String>) -> FailureMessage {
    FailureMessage {
        header: FailureHeader::UriFailure,
        fields: [
            ("URI".to_string(), uri.into()),
            ("Message".to_string(), message.into()),
        ]
        .into_iter()
        .collect(),
    }
}

/// Create a General Failure message indicating a general failure with the given `message`.
pub fn general_failure(message: impl Into<String>) -> FailureMessage {
    FailureMessage {
        header: FailureHeader::GeneralFailure,
        fields: [("Message".to_string(), message.into())]
            .into_iter()
            .collect(),
    }
}
