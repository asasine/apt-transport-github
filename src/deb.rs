//! Extract the `DEBIAN/control` file from a `.deb` package.
//!
//! A `.deb` is an `ar` archive containing `debian-binary`, `control.tar.*`, and
//! `data.tar.*`. This module extracts the raw control file content from the
//! `control.tar.*` member without needing the full `.deb`—only the bytes up
//! through the end of `control.tar.*` are required.

use crate::ar::ArArchive;
use std::io::Read;

/// Extract the `control` file content from a (possibly partial) `.deb` archive.
///
/// The provided bytes must include at least the `ar` header through the end of
/// the `control.tar.*` member. The `data.tar.*` member need not be present.
pub fn extract_control(deb_bytes: &[u8]) -> Result<String, Error> {
    let archive = ArArchive::new(deb_bytes).ok_or(Error::InvalidAr)?;
    let member = archive
        .find_by_prefix("control.tar")
        .ok_or(Error::MissingControlTar)?;

    let decoder: Box<dyn Read> = match member.name {
        n if n.ends_with(".gz") => Box::new(flate2::read::GzDecoder::new(member.data)),
        n if n.ends_with(".xz") => Box::new(xz2::read::XzDecoder::new(member.data)),
        n if n.ends_with(".zst") => Box::new(zstd::Decoder::new(member.data)?),
        "control.tar" => Box::new(member.data),
        _ => return Err(Error::UnsupportedCompression(member.name.to_string())),
    };

    let mut tar = tar::Archive::new(decoder);
    for entry in tar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path.as_os_str() == "./control" || path.as_os_str() == "control" {
            let mut control = String::new();
            entry.read_to_string(&mut control)?;
            // Ensure the control content ends with a newline so the index fields
            // are appended on a new line.
            if !control.ends_with('\n') {
                control.push('\n');
            }
            return Ok(control);
        }
    }

    Err(Error::MissingControlFile)
}

#[derive(Debug)]
pub enum Error {
    InvalidAr,
    MissingControlTar,
    UnsupportedCompression(String),
    MissingControlFile,
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAr => write!(f, "invalid ar archive"),
            Self::MissingControlTar => write!(f, "control.tar.* member not found"),
            Self::UnsupportedCompression(name) => {
                write!(f, "unsupported compression for member: {name}")
            }
            Self::MissingControlFile => write!(f, "control file not found in control.tar"),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a `.deb` (ar archive) with a gzip-compressed control.tar containing `./control`.
    fn build_deb_gz(control_content: &str) -> Vec<u8> {
        use flate2::write::GzEncoder;
        use std::io::Write;

        // Build the control.tar with a single ./control entry
        let mut tar_bytes = Vec::new();
        {
            let mut tar_builder = tar::Builder::new(&mut tar_bytes);
            let data = control_content.as_bytes();
            let mut header = tar::Header::new_gnu();
            header.set_path("./control").unwrap();
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar_builder.append(&header, data).unwrap();
            tar_builder.finish().unwrap();
        }

        // Gzip compress the tar
        let mut gz_bytes = Vec::new();
        {
            let mut encoder = GzEncoder::new(&mut gz_bytes, flate2::Compression::default());
            encoder.write_all(&tar_bytes).unwrap();
            encoder.finish().unwrap();
        }

        // Build the ar archive
        build_ar(&[("debian-binary", b"2.0\n"), ("control.tar.gz", &gz_bytes)])
    }

    /// Minimal ar archive builder (duplicated from ar::tests for independence).
    fn build_ar(members: &[(&str, &[u8])]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"!<arch>\n");
        for (name, data) in members {
            let name_field = format!("{name}/");
            buf.extend_from_slice(format!("{name_field:<16}").as_bytes());
            buf.extend_from_slice(b"0           ");
            buf.extend_from_slice(b"0     ");
            buf.extend_from_slice(b"0     ");
            buf.extend_from_slice(b"100644  ");
            buf.extend_from_slice(format!("{:<10}", data.len()).as_bytes());
            buf.extend_from_slice(b"`\n");
            buf.extend_from_slice(data);
            if data.len() % 2 != 0 {
                buf.push(b'\n');
            }
        }
        buf
    }

    #[test]
    fn extract_from_gzip_deb() {
        let control_content = "\
Package: example
Version: 1.0.0
Architecture: amd64
Maintainer: Test <test@test.com>
Description: A test package
";
        let deb = build_deb_gz(control_content);
        let result = extract_control(&deb).unwrap();
        assert_eq!(result, control_content);
    }

    #[test]
    fn trailing_newline_ensured() {
        let control_content = "Package: no-trailing-newline";
        let deb = build_deb_gz(control_content);
        let result = extract_control(&deb).unwrap();
        assert!(result.ends_with('\n'));
    }

    #[test]
    fn invalid_ar() {
        assert!(matches!(
            extract_control(b"not an ar archive"),
            Err(Error::InvalidAr)
        ));
    }

    #[test]
    fn missing_control_tar() {
        let deb = build_ar(&[("debian-binary", b"2.0\n"), ("data.tar.gz", b"fake")]);
        assert!(matches!(
            extract_control(&deb),
            Err(Error::MissingControlTar)
        ));
    }
}
