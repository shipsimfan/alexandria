use crate::system::OsFamily;
use std::borrow::Cow;

/// Keys containing potential names, ordered from better to worse
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OsVersionKey {
    /// The "PRETTY_NAME" key
    PrettyName,

    /// The "NAME" key
    Name,

    /// The "ID" key
    Id,
}

/// Get the full name of the operating system
pub fn os_name() -> String {
    let os_version = match read_os_version() {
        Some(os_version) => os_version,
        None => return OsFamily::CURRENT.to_string(),
    };

    // Parse it for a valid key
    let mut current_key = None;
    let mut current_name = None;
    for (key, value) in OsVersionIter::new(os_version.as_bytes()) {
        let key = match key {
            b"PRETTY_NAME" => OsVersionKey::PrettyName,
            b"NAME" => OsVersionKey::Name,
            b"ID" => OsVersionKey::Id,
            _ => continue,
        };

        if match current_key {
            Some(cur_key) => key < cur_key,
            None => true,
        } {
            current_key = Some(key);
            current_name = Some(value);
        }
    }

    match current_name {
        Some(current_name) => match String::from_utf8_lossy(current_name) {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        },
        None => OsFamily::CURRENT.to_string(),
    }
}

/// Attempt to read the `os-version` file
fn read_os_version() -> Option<String> {
    std::fs::read_to_string("/etc/os-release").ok()
}

/// An iterator over the values of an `os-version` file
struct OsVersionIter<'a> {
    /// The bytes that make up the file
    bytes: &'a [u8],

    /// The index of the next byte in the file
    index: usize,
}

impl<'a> OsVersionIter<'a> {
    /// Create a new [`OsVersionIter`]
    fn new(bytes: &'a [u8]) -> OsVersionIter<'a> {
        OsVersionIter { bytes, index: 0 }
    }

    /// Get the next byte in the file
    fn next_byte(&mut self) -> Option<u8> {
        if self.index == self.bytes.len() {
            return None;
        }

        let byte = self.bytes[self.index];
        self.index += 1;
        Some(byte)
    }
}

impl<'a> Iterator for OsVersionIter<'a> {
    type Item = (&'a [u8], &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        // Check for an empty line
        let mut key_start = self.index;
        let mut c = self.next_byte()?;
        while c == b'\n' {
            key_start = self.index;
            c = self.next_byte()?;
        }

        // Parse the key
        let mut key_end = self.index;
        while c != b'=' {
            key_end = self.index;
            match self.next_byte() {
                Some(new_c) => c = new_c,
                None => break,
            }
        }
        let key = &self.bytes[key_start..key_end];

        // Check for quotes
        let mut value_start = self.index;
        c = match self.next_byte() {
            Some(c) => c,
            None => return Some((key, &[])),
        };
        let quotes = c == b'"';
        if quotes {
            value_start = self.index;
            c = match self.next_byte() {
                Some(c) => c,
                None => return Some((key, &[])),
            };
        }

        // Parse value
        let mut value_end = self.index;
        while c != b'\n' && (!quotes || c != b'"') {
            value_end = self.index;
            match self.next_byte() {
                Some(new_c) => c = new_c,
                None => break,
            }
        }
        let value = &self.bytes[value_start..value_end];

        // Check for closing quote
        if quotes && c == b'"' {
            while c != b'\n' {
                match self.next_byte() {
                    Some(new_c) => c = new_c,
                    None => break,
                }
            }
        }

        // Return combo
        Some((key, value))
    }
}
