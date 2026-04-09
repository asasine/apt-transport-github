//! Client for the GitHub Releases API.
//!
//! Fetches release metadata and partial `.deb` assets (just enough to extract
//! `DEBIAN/control`) using HTTP Range requests.

use crate::ar::ArArchive;
use serde::Deserialize;

const INITIAL_RANGE_BYTES: usize = 64 * 1024; // 64 KiB

/// A GitHub release.
#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

/// A release asset.
#[derive(Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub size: u64,
    pub url: String,
    pub browser_download_url: String,
    pub content_type: String,

    /// Digest in the form `sha256:<hex>`, if available.
    #[serde(default)]
    pub digest: Option<String>,
}

impl Asset {
    /// Returns `true` if this asset is a `.deb` package.
    pub fn is_deb(&self) -> bool {
        self.content_type == "application/x-debian-package"
    }

    /// Extract the SHA-256 hex string from the `digest` field.
    pub fn sha256(&self) -> Option<&str> {
        self.digest
            .as_deref()
            .and_then(|d| d.strip_prefix("sha256:"))
    }
}

/// Fetch all releases for the given repository.
pub fn list_releases(user: &str, repo: &str) -> Result<Vec<Release>, Error> {
    let url = format!("https://api.github.com/repos/{user}/{repo}/releases");
    let response = ureq::get(&url)
        .header("Accept", "application/vnd.github+json")
        .call()?;

    let body = response.into_body().read_to_string()?;
    let releases: Vec<Release> = serde_json::from_str(&body).map_err(|e| Error::Io(e.into()))?;
    Ok(releases)
}

/// Fetch enough bytes of a `.deb` asset to extract `control.tar.*`.
///
/// Issues a speculative Range request for the first 64 KiB. If the ar headers
/// show that `control.tar.*` extends beyond the fetched bytes, a second Range
/// request fetches the remainder.
pub fn fetch_control_bytes(asset: &Asset) -> Result<Vec<u8>, Error> {
    let url = &asset.browser_download_url;

    // First request: speculative 64 KiB
    let initial_end = INITIAL_RANGE_BYTES.min(asset.size as usize) - 1;
    let mut buf = fetch_range(url, 0, initial_end)?;

    // Check if we have the full control.tar.* member
    if needs_more_bytes(&buf) {
        // Parse ar headers to find where control.tar.* ends
        if let Some(needed) = control_tar_end_offset(&buf)
            && needed > buf.len() {
                let mut rest = fetch_range(url, buf.len(), needed - 1)?;
                buf.append(&mut rest);
            }
    }

    Ok(buf)
}

/// Determine the byte offset (from start of archive) where `control.tar.*` ends.
fn control_tar_end_offset(data: &[u8]) -> Option<usize> {
    let archive = ArArchive::new(data)?;
    for member in archive {
        if member.name.starts_with("control.tar") {
            let end = member.data.as_ptr() as usize - data.as_ptr() as usize + member.data.len();
            return Some(end);
        }
    }
    None
}

/// Returns `true` if the fetched bytes do not fully contain `control.tar.*`.
fn needs_more_bytes(data: &[u8]) -> bool {
    let Some(archive) = ArArchive::new(data) else {
        return false;
    };

    for member in archive {
        if member.name.starts_with("control.tar") {
            // If the iterator yielded it, the data is fully contained.
            return false;
        }
    }

    // We either didn't find the member (truncated) or it wasn't present.
    true
}

fn fetch_range(url: &str, start: usize, end: usize) -> Result<Vec<u8>, Error> {
    let range = format!("bytes={start}-{end}");
    let response = ureq::get(url).header("Range", &range).call()?;

    let buf = response.into_body().read_to_vec()?;
    Ok(buf)
}

#[derive(Debug)]
pub enum Error {
    Http(Box<ureq::Error>),
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {e}"),
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Self::Http(Box::new(e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RELEASE_JSON: &str = r#"[
        {
            "tag_name": "v1.0.0",
            "assets": [
                {
                    "name": "example_1.0.0_amd64.deb",
                    "size": 123456,
                    "url": "https://api.github.com/repos/user/repo/releases/assets/1",
                    "browser_download_url": "https://github.com/user/repo/releases/download/v1.0.0/example_1.0.0_amd64.deb",
                    "content_type": "application/x-debian-package",
                    "digest": "sha256:abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
                },
                {
                    "name": "example_1.0.0_amd64.tar.gz",
                    "size": 654321,
                    "url": "https://api.github.com/repos/user/repo/releases/assets/2",
                    "browser_download_url": "https://github.com/user/repo/releases/download/v1.0.0/example_1.0.0_amd64.tar.gz",
                    "content_type": "application/gzip"
                }
            ]
        }
    ]"#;

    #[test]
    fn deserialize_releases() {
        let releases: Vec<Release> = serde_json::from_str(SAMPLE_RELEASE_JSON).unwrap();
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].tag_name, "v1.0.0");
        assert_eq!(releases[0].assets.len(), 2);
    }

    #[test]
    fn asset_is_deb() {
        let releases: Vec<Release> = serde_json::from_str(SAMPLE_RELEASE_JSON).unwrap();
        let assets = &releases[0].assets;
        assert!(assets[0].is_deb());
        assert!(!assets[1].is_deb());
    }

    #[test]
    fn asset_sha256() {
        let releases: Vec<Release> = serde_json::from_str(SAMPLE_RELEASE_JSON).unwrap();
        let assets = &releases[0].assets;
        assert_eq!(
            assets[0].sha256(),
            Some("abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890")
        );
        assert_eq!(assets[1].sha256(), None);
    }

    #[test]
    fn asset_sha256_missing_digest() {
        let asset = Asset {
            name: "test.deb".to_string(),
            size: 100,
            url: String::new(),
            browser_download_url: String::new(),
            content_type: String::new(),
            digest: None,
        };
        assert_eq!(asset.sha256(), None);
    }
}
