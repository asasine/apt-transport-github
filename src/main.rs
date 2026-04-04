use apt_transport_github::{
    Fds, Uri, apt,
    message::{
        self, Header, InformationalHeader, InformationalMessage, Message, OutboundHeader,
        RequestMessage,
    },
};
use core::fmt::Display;
use std::{
    io::{BufRead, Write},
    path::{Path, PathBuf},
};

#[derive(Default)]
struct Method {
    fds: Fds,
}

impl Method {
    /// Read an inbound request message from stdin until a blank line is encountered.
    fn receive_message(&mut self) -> std::io::Result<RequestMessage> {
        let mut buffer = String::new();
        let mut stdin = self.fds.stdin.lock();
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

    /// Send an outbound message to stdout.
    fn send_message(&mut self, message: Message<impl OutboundHeader>) -> std::io::Result<()> {
        writeln!(self.fds.stdout, "{}", message)?;
        self.fds.stdout.flush()
    }

    /// Send an emergency error message to stderr.
    ///
    /// This is not part of the protocol and does not require a specific format.
    /// Any message can be sent.
    #[allow(
        dead_code,
        reason = "Not always used depending on the state of the implementation."
    )]
    fn error(&mut self, message: impl Display) {
        let _ = writeln!(self.fds.stderr, "{}", message);
    }

    /// Run the method, which reads messages from stdin until EOF.
    ///
    /// The flow of messages is as follows:
    /// 1. Method sends out 100 on stdout.
    /// 2. APT sends 601 on stdin if 100 has Send-Config: true.
    /// 3. APT sends 600 on stdin.
    /// 4. Method responds with 200/201/400.
    ///
    /// Output to the user is done on stdout with 101/102. Emergency error messages
    /// are sent on stderr and aren't part of the protocol.
    fn run(mut self) -> std::io::Result<()> {
        let capabilities = InformationalMessage {
            header: InformationalHeader::Capabilities,
            fields: [("Version", "1.0"), ("Single-Instance", "true")]
                .into_iter()
                .collect(),
        };

        self.send_message(capabilities)?;

        while let Ok(message) = self.receive_message() {
            self.handle_message(message)?;
        }

        Ok(())
    }

    fn handle_message(&mut self, message: RequestMessage) -> std::io::Result<()> {
        match message.header {
            message::RequestHeader::UriAcquire => self.acquire_uri(message)?,
            message::RequestHeader::AuthorizationCredentials
            | message::RequestHeader::MediaChanged => {
                self.send_message(message::log(format!(
                    "Received unsupported message: {}",
                    message.header.display()
                )))?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Acquire the URI indicated by the message.
    fn acquire_uri(&mut self, message: RequestMessage) -> std::io::Result<()> {
        let uri = message.fields.find("URI").ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "URI field not provided",
        ))?;

        let uri = Uri::from_uri(uri).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "URI field is not in the expected format",
        ))?;

        if uri.path == "Packages" {
            let filename =
                message
                    .fields
                    .find("Filename")
                    .map(PathBuf::from)
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Filename field not provided",
                    ))?;

            return self.acquire_packages(uri, &filename);
        }

        // all other URIs are unsupported
        self.send_message(message::uri_failure(uri.original, "Not supported"))
    }

    /// Acquire the `Packages` file from the given URI and save it to the given filename.
    fn acquire_packages(&mut self, uri: Uri, filename: &Path) -> std::io::Result<()> {
        self.send_message(message::status(format!(
            "Fetching list of packages from {}",
            uri.display_repository(),
        )))?;

        let packages = apt::Packages::from_uri(uri);
        self.send_message(message::uri_failure(
            &packages.uri().original,
            "Packages not supported yet",
        ))
    }
}

fn main() -> std::io::Result<()> {
    let method = Method::default();
    method.run()
}
