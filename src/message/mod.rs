//! Message types for communicating with APT as an APT transport method.
//!
//! The first line of each message is called the message header. The first 3 digits
//! (called the Status Code) have the usual meaning found in the http protocol.
//!
//! - 1xx is informational: [`InformationalHeader`]
//! - 2xx is successful: [`SuccessHeader`]
//! - 4xx is failure: [`FailureHeader`]
//! - The 6xx series is used to specify things sent to the method: [`RequestHeader`]
//!
//! After the status code is an informational string provided for visual debugging.
//!
//! Only the 6xx series of status codes is sent TO the method. Furthermore the
//! method may not emit status codes in the 6xx range. The Codes [`FailureHeader::AuthorizationRequired`]
//! and [`FailureHeader::MediaFailure`] require that the method continue reading all other
//! 6xx codes until the proper [`RequestHeader::AuthorizationCredentials`]/[`RequestHeader::MediaChanged`]
//! code is received. This means the method must be capable of handling an unlimited
//! number of [`RequestHeader::UriAcquire`] messages.
//!
//! The flow of messages starts with the method sending out a [`InformationalHeader::Capabilities`]
//! and APT sending out a [`RequestHeader::Configuration`]. After that APT begins sending
//! [`RequestHeader::UriAcquire`] messages and the method sends out [`SuccessHeader::UriStart`], [`SuccessHeader::UriDone`]
//! or [`FailureHeader::UriFailure`]. No synchronization is performed, it is expected that
//! APT will send [`SuccessHeader::UriAcquire`] messages at -any- time and that the method
//!  should queue the messages. This allows methods like http to pipeline requests
//! to the remote server. It should be noted however that APT will buffer messages
//! so it is not necessary for the method to be constantly ready to receive them.

mod fields;
mod header;
mod owned;
mod r#ref;
mod factories;

use fields::Fields;
pub use header::{
    FailureHeader, Header, InboundHeader, InformationalHeader, OutboundHeader, RequestHeader,
    SuccessHeader,
};
pub use owned::Message;
pub use r#ref::RefMessage;
pub use factories::*;

/// An informational message sent by the method to the APT parent process.
pub type InformationalMessage = Message<InformationalHeader>;

/// A success message sent by the method to the APT parent process.
pub type SuccessMessage = Message<SuccessHeader>;

/// A failure message sent by the method to the APT parent process.
pub type FailureMessage = Message<FailureHeader>;

/// A request message sent by the APT parent process to the method.
pub type RequestMessage = Message<RequestHeader>;
