//! # Pass 
//! `pass` contains primitives for exporting pass <https://passwordstore.org>
//! entries into structured data.
//!
//! `pass` password entries utilize an informal schema. By convention, many
//! consumers of `pass` data use the `url` and `login` directives.
use serde::{Deserialize, Serialize};

/// An entry in the password store
#[derive(Serialize, Debug, Deserialize)]
pub struct Entry {
    pub name: String,
    pub password: Option<String>,
    pub login: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
}

impl Entry {
    /// Decode a password store entry from an &str input
    pub fn from_str(name: &str, data: &str) -> Result<Entry, &'static str> {
        if name.len() < 1 {
            return Err("invalid name");
        }
        if data.len() < 1 {
            return Err("invalid data");
        }

        let mut entry = Entry {
            name: name.to_string(),
            login: None,
            password: None,
            url: None,
            notes: None,
        };

        let mut note_content = String::new();

        let lines = data.split("\n");

        for (i, line) in lines.enumerate() {
            if i == 0 {
                entry.password = Some(line.to_string());
                continue;
            }
            if line.starts_with("url:") {
                entry.url = Some(line[4..].trim().to_string());
                continue;
            }
            if line.starts_with("login:") {
                entry.login = Some(line[6..].trim().to_string());
                continue;
            }

            note_content.push_str(line);
            note_content.push_str("\n");
        }

        if note_content.len() > 1 {
            entry.notes = Some(note_content);
        }

        Ok(entry)
    }

    /// Decode a password store entry from utf8 input.
    ///
    ///
    /// # Example:
    /// This is handy for piping the return from stdout.
    /// ```
    /// let name = "myEmail"
    /// let output = Command::new("pass")
    ///    .arg(&name)
    ///    .stdout(Stdio::piped())
    ///    .output()
    ///    .expect("command failed");
    ///
    /// let entry = pass::Entry.from_utf8(&output.stdout).unwrap();
    /// ```
    pub fn from_utf8(entry: &str, data: &[u8]) -> Result<Entry, &'static str> {
        let input = std::str::from_utf8(data);
        match input {
            Err(_) => return Err("invalid data"),
            Ok(data) => return Entry::from_str(entry, &data),
        }
    }
}

