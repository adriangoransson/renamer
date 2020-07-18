use crate::{args::Increment, errors::RenameError};
use regex::Regex;
use std::{
    io,
    path::{Path, PathBuf},
};

pub struct FileRenamer {
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum IncrementPosition {
    Prefix,
    Suffix,
}

impl FileRenamer {
    /// Creates a new builder with the received path.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        FileRenamer {
            path: path.as_ref().to_owned(),
        }
    }

    fn file_name(&self) -> io::Result<String> {
        let name = self
            .path
            .file_name()
            .expect("FileRenamer was created with an invalid file.");

        let s = name.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("File name {:?} contains invalid UTF-8.", name),
            )
        })?;

        Ok(s.to_string())
    }

    /// Apply the `regex=replacement` patterns provided to the file name in sequence.
    pub fn apply_patterns(
        &mut self,
        replace_all: bool,
        patterns: &[(Regex, String)],
    ) -> Result<&mut Self, RenameError> {
        let replace = if replace_all {
            Regex::replace_all
        } else {
            Regex::replace
        };

        let mut file_name = self.file_name()?;

        for (regex, replacement) in patterns {
            let rep = replacement.as_str();
            file_name = replace(regex, &file_name, rep).to_string();
        }

        self.path.set_file_name(file_name);

        Ok(self)
    }

    /// Takes a position (`{pre,suf}fix`), an Increment struct with the width and starting index.
    /// The count specifies the current index or amount to add to the starting index.
    ///
    /// It will try to respect the naming of hidden files (preceding dot) so that they stay hidden.
    /// Extensions should also be preserved.
    pub fn increment(
        &mut self,
        position: IncrementPosition,
        increment: Increment,
        count: usize,
    ) -> Result<&mut Self, RenameError> {
        let mut file_name = self.file_name()?;

        let inc = format!(
            "{:0width$}",
            increment.start + count,
            width = increment.width
        );

        file_name = interpolate_increment(file_name, &inc, position);

        self.path.set_file_name(file_name);

        Ok(self)
    }

    pub fn finish(self) -> PathBuf {
        self.path
    }
}

fn interpolate_increment(mut name: String, inc: &str, position: IncrementPosition) -> String {
    // Respect hidden files.
    let start_index = if name.starts_with('.') { 1 } else { 0 };

    match position {
        IncrementPosition::Prefix => name.insert_str(start_index, &inc),
        IncrementPosition::Suffix => {
            let last_dot = name.rfind('.');

            match last_dot {
                Some(i) if i > start_index => name.insert_str(i, &inc),
                _ => name.push_str(&inc),
            }
        }
    }

    name
}

#[cfg(test)]
mod tests {
    use super::{interpolate_increment, IncrementPosition};

    #[test]
    fn interpolate_hidden() {
        assert_eq!(
            ".vimrc123",
            interpolate_increment(".vimrc".to_string(), "123", IncrementPosition::Suffix)
        );

        assert_eq!(
            ".003hidden.ext",
            interpolate_increment(".hidden.ext".to_string(), "003", IncrementPosition::Prefix)
        );
    }
}
