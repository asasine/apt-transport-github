use super::{Header, OutboundHeader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuccessHeader {
    /// 200 URI Start - URI is starting acquire
    ///
    /// Indicates the `URI` is starting to be transfered. The `URI` is specified
    /// along with stats about the file itself.
    ///
    /// Fields:
    /// - URI
    /// - Size
    /// - Last-Modified
    /// - Resume-Point
    UriStart,

    /// 201 URI Done - URI is finished acquire
    ///
    /// Indicates that a `URI` has completed being transfered. It is possible to
    /// specify a [`Self::UriDone`] without a [`Self::UriStart`] which would mean
    /// no data was transfered but the file is now available. A `Filename` field
    /// is specified when the `URI` is directly available in the local pathname
    /// space. APT will either directly use that file or copy it into another location.
    /// It is possible to return `Alt-*` fields to indicate that another possibility
    /// for the `URI` has been found in the local pathname space. This is done if
    /// a decompressed version of a .gz file is found.
    ///
    /// Fields:
    /// - URI
    /// - Size
    /// - Last-Modified
    /// - Filename
    /// - MD5-Hash
    UriDone,
}

impl Header for SuccessHeader {
    fn code(&self) -> u16 {
        match self {
            Self::UriStart => 200,
            Self::UriDone => 201,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::UriStart => "URI Start",
            Self::UriDone => "URI Done",
        }
    }
}

impl OutboundHeader for SuccessHeader {}
