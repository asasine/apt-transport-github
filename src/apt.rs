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
struct Package {
    package: String,
    architecture: String,
    version: String,
    priority: String,
    section: String,
    maintainer: String,
    installed_size: String,
    depends: String,
    recommends: String,
    filename: String,
    size: String,
    md5sum: String,
    sha1: String,
    sha256: String,
    sha512: String,
    homepage: String,
    description: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Package: {}", self.package)?;
        writeln!(f, "Architecture: {}", self.architecture)?;
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Priority: {}", self.priority)?;
        writeln!(f, "Section: {}", self.section)?;
        writeln!(f, "Maintainer: {}", self.maintainer)?;
        writeln!(f, "Installed-Size: {}", self.installed_size)?;
        writeln!(f, "Depends: {}", self.depends)?;
        writeln!(f, "Recommends: {}", self.recommends)?;
        writeln!(f, "Filename: {}", self.filename)?;
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "MD5sum: {}", self.md5sum)?;
        writeln!(f, "SHA1: {}", self.sha1)?;
        writeln!(f, "SHA256: {}", self.sha256)?;
        writeln!(f, "SHA512: {}", self.sha512)?;
        writeln!(f, "Homepage: {}", self.homepage)?;
        write!(f, "Description: {}", self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packages_file() {
        let packages = vec![
            Package {
                package: "example1".to_string(),
                architecture: "amd64".to_string(),
                version: "1.0.0".to_string(),
                priority: "optional".to_string(),
                section: "utils".to_string(),
                maintainer: "Example Maintainer <example@example.com>".to_string(),
                installed_size: "1024".to_string(),
                depends: "libexample1 (>= 1.0), libexample2 (>= 2.0)".to_string(),
                recommends: "libexample3 (>= 3.0)".to_string(),
                filename: "pool/main/e/example/example1_1.0.0_amd64.deb".to_string(),
                size: "2048".to_string(),
                md5sum: "d41d8cd98f00b204e980099ecf8427e".to_string(),
                sha1: "da39a3ee5e6b4b0d3255bfef95601890afd80709".to_string(),
                sha256: "e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5".to_string(),
                sha512: "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d6c15b0f00a08".to_string(),
                homepage: "https://example.com".to_string(),
                description: "An example package".to_string(),
            },
            Package {
                package: "example2".to_string(),
                architecture: "amd64".to_string(),
                version: "1.0.0".to_string(),
                priority: "optional".to_string(),
                section: "utils".to_string(),
                maintainer: "Example Maintainer <example@example.com>".to_string(),
                installed_size: "1024".to_string(),
                depends: "libexample1 (>= 1.0), libexample2 (>= 2.0)".to_string(),
                recommends: "libexample3 (>= 3.0)".to_string(),
                filename: "pool/main/e/example/example2_1.0.0_amd64.deb".to_string(),
                size: "2048".to_string(),
                md5sum: "d41d8cd98f00b204e980099ecf8427e".to_string(),
                sha1: "da39a3ee5e6b4b0d3255bfef95601890afd80709".to_string(),
                sha256: "e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5".to_string(),
                sha512: "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d6c15b0f00a08".to_string(),
                homepage: "https://example.com".to_string(),
                description: "An example package".to_string(),
            }
        ];

        let packages_file = PackagesFile(&packages);

        let expected = r#"Package: example1
Architecture: amd64
Version: 1.0.0
Priority: optional
Section: utils
Maintainer: Example Maintainer <example@example.com>
Installed-Size: 1024
Depends: libexample1 (>= 1.0), libexample2 (>= 2.0)
Recommends: libexample3 (>= 3.0)
Filename: pool/main/e/example/example1_1.0.0_amd64.deb
Size: 2048
MD5sum: d41d8cd98f00b204e980099ecf8427e
SHA1: da39a3ee5e6b4b0d3255bfef95601890afd80709
SHA256: e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5
SHA512: cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d6c15b0f00a08
Homepage: https://example.com
Description: An example package

Package: example2
Architecture: amd64
Version: 1.0.0
Priority: optional
Section: utils
Maintainer: Example Maintainer <example@example.com>
Installed-Size: 1024
Depends: libexample1 (>= 1.0), libexample2 (>= 2.0)
Recommends: libexample3 (>= 3.0)
Filename: pool/main/e/example/example2_1.0.0_amd64.deb
Size: 2048
MD5sum: d41d8cd98f00b204e980099ecf8427e
SHA1: da39a3ee5e6b4b0d3255bfef95601890afd80709
SHA256: e3b0c44298fc1c149afbf49daeaeeae4e8f1b2b0b822cd15d6bf511a6f5
SHA512: cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d6c15b0f00a08
Homepage: https://example.com
Description: An example package
"#;
        assert_eq!(packages_file.to_string(), expected);
    }
}
