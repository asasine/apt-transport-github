//! Contains types to create an APT Repository from a GitHub repository.

use crate::{Uri, utilities::Rfc822};
use core::fmt::Display;

/// Contains the packages in a repository, for serializing to a `Packages` file.
pub struct Packages {
    uri: Uri,
    packages: Vec<Package>,
}

impl Packages {
    pub fn from_uri(uri: Uri) -> Self {
        // TODO: fetch packages from the repository releases
        // TODO: create a `Package` for each release asset with .deb extension
        Self {
            uri,
            packages: Default::default(),
        }
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
