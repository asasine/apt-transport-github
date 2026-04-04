use core::fmt::Display;
use std::sync::LazyLock;

pub struct Uri {
    pub original: String,
    pub user: String,
    pub repo: String,
    pub path: String,
}

impl Uri {
    /// Parse a URI from the `URI` field of a `600 URI-Acquire` message.
    ///
    /// The URI is expected to be in the format: `github://{user}/{repo}/./{path}`
    ///
    /// If the URI is not in the expected format, [`None`] is returned.
    pub fn from_uri(uri: impl Into<String>) -> Option<Self> {
        let uri = uri.into();
        static GITHUB_URI_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
            regex::Regex::new(r"^github://(?P<user>[^/]+)/(?P<repo>[^/]+)/\./(?P<path>.+)$")
                .unwrap()
        });

        let captures = GITHUB_URI_REGEX.captures(&uri)?;
        let user = captures.name("user")?.as_str().to_string();
        let repo = captures.name("repo")?.as_str().to_string();
        let path = captures.name("path")?.as_str().to_string();
        Some(Self {
            original: uri,
            user,
            repo,
            path,
        })
    }

    pub fn display_repository(&self) -> impl Display {
        struct RepoDisplay<'a> {
            user: &'a str,
            repo: &'a str,
        }

        impl Display for RepoDisplay<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}/{}", self.user, self.repo)
            }
        }

        RepoDisplay {
            user: &self.user,
            repo: &self.repo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_uri() {
        let uri = "github://user/repo/./path/to/file";
        let parsed = Uri::from_uri(uri).unwrap();
        assert_eq!(parsed.original, uri);
        assert_eq!(parsed.user, "user");
        assert_eq!(parsed.repo, "repo");
        assert_eq!(parsed.path, "path/to/file");
    }
}
