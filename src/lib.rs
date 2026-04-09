use core::{error::Error, fmt::Display};
use std::io::{BufRead, Stderr, Stdin, Stdout, Write};

pub mod apt;
pub mod ar;
pub mod deb;
pub mod github;
pub mod message;
mod uri;
pub(crate) mod utilities;

pub use uri::Uri;

use crate::message::{Message, OutboundHeader, RequestMessage};

/// File descriptors for an APT transport method.
pub struct Fds {
    /// The input stream for receiving messages from APT.
    pub stdin: Stdin,

    /// The output stream for sending messages to APT.
    pub stdout: Stdout,

    /// The error stream for sending emergency error messages to APT and the user.
    pub stderr: Stderr,
}

/// The default implementation of [`Fds`] uses the standard input, output, and error streams.
impl Default for Fds {
    fn default() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            stderr: std::io::stderr(),
        }
    }
}

/// An APT transport method, which can receive request messages and send response messages.
pub trait Method {
    type Error: Error;

    /// Read an inbound request message from stdin until a blank line is encountered.
    fn receive_message(&mut self) -> Result<RequestMessage, Self::Error>;

    /// Send an outbound message.
    fn send_message(&mut self, message: Message<impl OutboundHeader>) -> Result<(), Self::Error>;

    /// Send an emergency error message.
    ///
    /// This is not part of the protocol and does not require a specific format.
    /// Any message can be sent.
    fn error(&mut self, message: impl Display);
}

impl<M: Method> Method for &mut M {
    type Error = M::Error;

    fn receive_message(&mut self) -> Result<RequestMessage, Self::Error> {
        (*self).receive_message()
    }

    fn send_message(&mut self, message: Message<impl OutboundHeader>) -> Result<(), Self::Error> {
        (*self).send_message(message)
    }

    fn error(&mut self, message: impl Display) {
        (*self).error(message)
    }
}

impl Method for Fds {
    type Error = std::io::Error;

    fn receive_message(&mut self) -> Result<RequestMessage, Self::Error> {
        let mut buffer = String::new();
        let mut stdin = self.stdin.lock();
        loop {
            let bytes_read = stdin.read_line(&mut buffer)?;
            if bytes_read == 0 {
                // EOF
                break;
            }

            if bytes_read == 1 && buffer.as_bytes().last().is_some_and(|&last| last == b'\n') {
                // Blank line indicates end of message
                break;
            }
        }

        let message = buffer.trim().parse().map_err(|()| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid message format")
        })?;

        Ok(message)
    }

    fn send_message(&mut self, message: Message<impl OutboundHeader>) -> Result<(), Self::Error> {
        writeln!(self.stdout, "{}", message)?;
        self.stdout.flush()
    }

    fn error(&mut self, message: impl Display) {
        let _ = writeln!(self.stderr, "{}", message);
    }
}
