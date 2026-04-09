//! Minimal parser for the Unix `ar` archive format used by `.deb` packages.

use std::str;

const MAGIC: &[u8] = b"!<arch>\n";
const HEADER_SIZE: usize = 60;

/// A member within an `ar` archive.
pub struct ArMember<'a> {
    /// The member name, trimmed of trailing spaces and the `/` suffix.
    pub name: &'a str,

    /// The member's content bytes.
    pub data: &'a [u8],
}

/// An iterator over members of an `ar` archive.
pub struct ArArchive<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> ArArchive<'a> {
    /// Parse an `ar` archive from the given bytes.
    ///
    /// Returns `None` if the archive does not start with valid `ar` magic.
    pub fn new(data: &'a [u8]) -> Option<Self> {
        if data.len() < MAGIC.len() || &data[..MAGIC.len()] != MAGIC {
            return None;
        }

        Some(Self {
            data,
            offset: MAGIC.len(),
        })
    }

    /// Find the first member whose name starts with the given prefix.
    pub fn find_by_prefix(self, prefix: &str) -> Option<ArMember<'a>> {
        self.into_iter().find(|m| m.name.starts_with(prefix))
    }
}

impl<'a> Iterator for ArArchive<'a> {
    type Item = ArMember<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + HEADER_SIZE > self.data.len() {
            return None;
        }

        let header = &self.data[self.offset..self.offset + HEADER_SIZE];

        // Validate the header magic trailer: bytes 58..60 must be "`\n"
        if &header[58..60] != b"`\n" {
            return None;
        }

        let name = str::from_utf8(&header[0..16])
            .ok()?
            .trim()
            .trim_end_matches('/');
        let size: usize = str::from_utf8(&header[48..58]).ok()?.trim().parse().ok()?;

        let data_start = self.offset + HEADER_SIZE;
        let data_end = data_start + size;

        if data_end > self.data.len() {
            return None;
        }

        let data = &self.data[data_start..data_end];

        // Members are padded to even byte boundaries.
        self.offset = data_end + (size % 2);

        Some(ArMember { name, data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal ar archive in memory.
    fn build_ar(members: &[(&str, &[u8])]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(MAGIC);
        for (name, data) in members {
            // 16-byte name field: "name/" padded with spaces
            let name_field = format!("{name}/");
            buf.extend_from_slice(format!("{name_field:<16}").as_bytes());
            // 12-byte mtime
            buf.extend_from_slice(b"0           ");
            // 6-byte owner
            buf.extend_from_slice(b"0     ");
            // 6-byte group
            buf.extend_from_slice(b"0     ");
            // 8-byte mode
            buf.extend_from_slice(b"100644  ");
            // 10-byte size
            buf.extend_from_slice(format!("{:<10}", data.len()).as_bytes());
            // 2-byte trailer
            buf.extend_from_slice(b"`\n");
            // data
            buf.extend_from_slice(data);
            // padding for odd sizes
            if data.len() % 2 != 0 {
                buf.push(b'\n');
            }
        }
        buf
    }

    #[test]
    fn invalid_magic() {
        assert!(ArArchive::new(b"not an ar archive").is_none());
    }

    #[test]
    fn empty_archive() {
        let ar = build_ar(&[]);
        let archive = ArArchive::new(&ar).unwrap();
        assert_eq!(archive.count(), 0);
    }

    #[test]
    fn single_member() {
        let ar = build_ar(&[("hello.txt", b"hello world")]);
        let mut archive = ArArchive::new(&ar).unwrap();
        let member = archive.next().unwrap();
        assert_eq!(member.name, "hello.txt");
        assert_eq!(member.data, b"hello world");
        assert!(archive.next().is_none());
    }

    #[test]
    fn multiple_members() {
        let ar = build_ar(&[
            ("debian-binary", b"2.0\n"),
            ("control.tar.gz", b"fake-control-data"),
            ("data.tar.gz", b"fake-data-data!"),
        ]);
        let members: Vec<_> = ArArchive::new(&ar).unwrap().collect();
        assert_eq!(members.len(), 3);
        assert_eq!(members[0].name, "debian-binary");
        assert_eq!(members[0].data, b"2.0\n");
        assert_eq!(members[1].name, "control.tar.gz");
        assert_eq!(members[1].data, b"fake-control-data");
        assert_eq!(members[2].name, "data.tar.gz");
        assert_eq!(members[2].data, b"fake-data-data!");
    }

    #[test]
    fn odd_size_padding() {
        // First member has odd size (5 bytes), second should still parse
        let ar = build_ar(&[("odd", b"12345"), ("even", b"abcd")]);
        let members: Vec<_> = ArArchive::new(&ar).unwrap().collect();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].data, b"12345");
        assert_eq!(members[1].data, b"abcd");
    }

    #[test]
    fn find_by_prefix() {
        let ar = build_ar(&[
            ("debian-binary", b"2.0\n"),
            ("control.tar.zst", b"fake-control"),
            ("data.tar.zst", b"fake-data"),
        ]);
        let archive = ArArchive::new(&ar).unwrap();
        let member = archive.find_by_prefix("control.tar").unwrap();
        assert_eq!(member.name, "control.tar.zst");
        assert_eq!(member.data, b"fake-control");
    }

    #[test]
    fn find_by_prefix_missing() {
        let ar = build_ar(&[("debian-binary", b"2.0\n")]);
        let archive = ArArchive::new(&ar).unwrap();
        assert!(archive.find_by_prefix("control.tar").is_none());
    }

    #[test]
    fn truncated_data() {
        // Archive where header claims more data than is available
        let mut ar = build_ar(&[("file.txt", b"hello")]);
        ar.truncate(ar.len() - 3); // chop off part of the data
        let mut archive = ArArchive::new(&ar).unwrap();
        assert!(archive.next().is_none());
    }
}
