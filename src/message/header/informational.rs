use super::{Header, OutboundHeader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InformationalHeader {
    /// 100 Capabilities - Method capabilities
    ///
    /// Displays the capabilities of the method. Methods should set the `pipeline`
    /// bit if their underlying protocol supports pipelining. The only known method
    /// that does support pipelining is http.
    ///
    /// Fields:
    /// - Version
    /// - Single-Instance
    /// - Pre-Scan
    /// - Pipeline
    /// - Send-Config
    /// - Needs-Cleanup
    Capabilities,

    /// 101 Log - General Logging
    ///
    /// A log message may be printed to the screen if debugging is enabled. This
    /// is only for debugging the method.
    ///
    /// Fields:
    /// - Message
    Log,

    /// 102 Status - Inter-URI status reporting (login progress)
    ///
    /// `Message` gives a progress indication for the method. It can be used to
    /// show pre-transfer status for Internet type methods.
    ///
    /// Fields:
    /// - Message
    Status,
}

impl Header for InformationalHeader {
    fn code(&self) -> u16 {
        match self {
            Self::Capabilities => 100,
            Self::Log => 101,
            Self::Status => 102,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Capabilities => "Capabilities",
            Self::Log => "Log",
            Self::Status => "Status",
        }
    }
}

impl OutboundHeader for InformationalHeader {}
