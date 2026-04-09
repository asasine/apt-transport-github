use apt_transport_github::{
    Fds, Method, Uri, apt, github,
    message::{
        self, Header, InformationalHeader, InformationalMessage, Message, OutboundHeader,
        RequestMessage,
    },
};
use core::fmt::Display;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct AptMethod {
    fds: Fds,
}

/// Implement the APT transport method for [`Method`] by delegating to the [`Self::fds`].
impl Method for AptMethod {
    type Error = <Fds as Method>::Error;

    fn receive_message(&mut self) -> Result<RequestMessage, Self::Error> {
        self.fds.receive_message()
    }

    fn send_message(&mut self, message: Message<impl OutboundHeader>) -> Result<(), Self::Error> {
        self.fds.send_message(message)
    }

    fn error(&mut self, message: impl Display) {
        self.fds.error(message)
    }
}

impl AptMethod {
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

        if let Some(asset_id) = uri.path.strip_prefix("assets/").map(str::to_string) {
            let filename =
                message
                    .fields
                    .find("Filename")
                    .map(PathBuf::from)
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Filename field not provided",
                    ))?;

            return self.acquire_asset(uri, &asset_id, &filename);
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

        let uri_original = uri.original.clone();
        match apt::Packages::from_uri(uri) {
            Ok(packages) => {
                std::fs::write(filename, packages.into_packages_file())?;
                self.send_message(message::uri_done(&uri_original, filename.to_string_lossy()))
            }
            Err(e) => self.send_message(message::uri_failure(&uri_original, e.to_string())),
        }
    }

    /// Acquire a release asset (`.deb` file) and save it to the given filename.
    fn acquire_asset(
        &mut self,
        uri: Uri,
        asset_id: &str,
        filename: &Path,
    ) -> std::io::Result<()> {
        let asset_id: u64 = asset_id.parse().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid asset ID")
        })?;

        self.send_message(message::uri_start(&uri.original))?;

        let uri_original = uri.original.clone();
        match github::download_asset(&uri.owner, &uri.repo, asset_id) {
            Ok(bytes) => {
                std::fs::write(filename, bytes)?;
                self.send_message(message::uri_done(&uri_original, filename.to_string_lossy()))
            }
            Err(e) => self.send_message(message::uri_failure(&uri_original, e.to_string())),
        }
    }
}

fn main() -> std::io::Result<()> {
    let method = AptMethod::default();
    method.run()
}
