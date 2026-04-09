use core::fmt::Display;
use std::sync::LazyLock;

/// A parsed URI from the `URI` field of a `600 URI Acquire` message.
pub struct Uri {
    /// The original URI string as received in the message.
    pub original: String,

    /// The GitHub username or organization owning the repository.
    pub owner: String,

    /// The GitHub repository name.
    pub repo: String,

    /// The path to the resource within the repository.
    pub path: String,
}

impl Uri {
    /// Parse a URI from the `URI` field of a `600 URI Acquire` message.
    ///
    /// The URI is expected to be in the format: `github://{user}/{repo}/./{path}`
    ///
    /// If the URI is not in the expected format, [`None`] is returned.
    pub fn from_uri(uri: impl Into<String>) -> Option<Self> {
        let uri = uri.into();
        static GITHUB_URI_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
            regex::Regex::new(r"^github://(?P<owner>[^/]+)/(?P<repo>[^/]+)/\./(?P<path>.+)$")
                .unwrap()
        });

        let captures = GITHUB_URI_REGEX.captures(&uri)?;
        let owner = captures.name("owner")?.as_str().to_string();
        let repo = captures.name("repo")?.as_str().to_string();
        let path = captures.name("path")?.as_str().to_string();
        Some(Self {
            original: uri,
            owner,
            repo,
            path,
        })
    }

    pub fn display_repository(&self) -> impl Display {
        struct RepoDisplay<'a> {
            owner: &'a str,
            repo: &'a str,
        }

        impl Display for RepoDisplay<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}/{}", self.owner, self.repo)
            }
        }

        RepoDisplay {
            owner: &self.owner,
            repo: &self.repo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_uri() {
        let uri = "github://owner/repo/./path/to/file";
        let parsed = Uri::from_uri(uri).unwrap();
        assert_eq!(parsed.original, uri);
        assert_eq!(parsed.owner, "owner");
        assert_eq!(parsed.repo, "repo");
        assert_eq!(parsed.path, "path/to/file");
    }

    #[test]
    fn from_uri_asset() {
        let uri = "github://owner/repo/./assets/12345";
        let parsed = Uri::from_uri(uri).unwrap();
        assert_eq!(parsed.owner, "owner");
        assert_eq!(parsed.repo, "repo");
        assert_eq!(parsed.path, "assets/12345");
    }
}
