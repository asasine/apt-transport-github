//! Contains types to create an APT Repository from a GitHub repository.

use crate::{Uri, deb, github, utilities::Rfc822};
use core::fmt::Display;

/// Contains the packages in a repository, for serializing to a `Packages` file.
pub struct Packages {
    uri: Uri,
    packages: Vec<Package>,
}

impl Packages {
    pub fn from_uri(uri: Uri) -> Result<Self, Error> {
        let releases = github::list_releases(&uri.owner, &uri.repo)?;
        let mut packages = Vec::new();

        for release in &releases {
            for asset in &release.assets {
                if !asset.is_deb() {
                    continue;
                }

                let bytes = github::fetch_control_bytes(asset)?;
                let control = deb::extract_control(&bytes)?;
                let sha256 = asset.sha256().unwrap_or_default().to_string();

                packages.push(Package {
                    control,
                    filename: asset.url.clone(),
                    size: asset.size.to_string(),
                    sha256,
                });
            }
        }

        Ok(Self { uri, packages })
    }

    pub const fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Create a `Packages` file from the given repository URI.
    pub fn into_packages_file(&self) -> String {
        PackagesFile(&self.packages).to_string()
    }
}

struct PackagesFile<'a>(&'a Vec<Package>);
impl Display for PackagesFile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Rfc822(self.0).fmt(f)
    }
}

/// An individual package in a repository.
///
/// The `control` field holds the verbatim content of the `.deb`'s `DEBIAN/control`
/// file. The index-specific fields (`Filename`, `Size`, `SHA256`) are appended
/// when serializing to a `Packages` stanza.
struct Package {
    /// The raw content of the `DEBIAN/control` file from the `.deb`.
    control: String,

    /// Path to the `.deb` file relative to the repository root.
    filename: String,

    /// Size of the `.deb` file in bytes in its compressed/packaged form.
    size: String,

    /// SHA-256 checksum of the `.deb` file.
    sha256: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.control)?;
        writeln!(f, "Filename: {}", self.filename)?;
        writeln!(f, "Size: {}", self.size)?;
        write!(f, "SHA256: {}", self.sha256)
    }
}

#[derive(Debug)]
pub enum Error {
    Github(github::Error),
    Deb(deb::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Github(e) => write!(f, "GitHub API error: {e}"),
            Self::Deb(e) => write!(f, "deb extraction error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<github::Error> for Error {
    fn from(e: github::Error) -> Self {
        Self::Github(e)
    }
}

impl From<deb::Error> for Error {
    fn from(e: deb::Error) -> Self {
        Self::Deb(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packages_file() {
        let packages = vec![
            Package {
                control: r#"Package: example1
Version: 1.0.0
Architecture: amd64
Maintainer: Example Maintainer <example@example.com>
Description: An example package
Installed-Size: 1024
Depends: libexample1 (>= 1.0), libexample2 (>= 2.0)
Recommends: libexample3 (>= 3.0)
"#
                .to_string(),
                filename: "pool/main/e/example/example1_1.0.0_amd64.deb".to_string(),
                size: "2048".to_string(),
                sha256: "e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5".to_string(),
            },
            Package {
                control: r#"Package: example2
Version: 1.0.0
Architecture: amd64
Maintainer: Example Maintainer <example@example.com>
Description: An example package
Multi-Arch: same
"#
                .to_string(),
                filename: "pool/main/e/example/example2_1.0.0_amd64.deb".to_string(),
                size: "2048".to_string(),
                sha256: "e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5".to_string(),
            },
        ];

        let packages_file = PackagesFile(&packages);

        let expected = "\
Package: example1
Version: 1.0.0
Architecture: amd64
Maintainer: Example Maintainer <example@example.com>
Description: An example package
Installed-Size: 1024
Depends: libexample1 (>= 1.0), libexample2 (>= 2.0)
Recommends: libexample3 (>= 3.0)
Filename: pool/main/e/example/example1_1.0.0_amd64.deb
Size: 2048
SHA256: e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5

Package: example2
Version: 1.0.0
Architecture: amd64
Maintainer: Example Maintainer <example@example.com>
Description: An example package
Multi-Arch: same
Filename: pool/main/e/example/example2_1.0.0_amd64.deb
Size: 2048
SHA256: e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5
";
        assert_eq!(packages_file.to_string(), expected);
    }
}
