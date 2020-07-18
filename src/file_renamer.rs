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

        // Respect hidden files.
        let start_index = if file_name.starts_with('.') { 1 } else { 0 };

        match position {
            IncrementPosition::Prefix => file_name.insert_str(start_index, &inc),
            IncrementPosition::Suffix => {
                let last_dot = file_name.rfind('.');

                match last_dot {
                    Some(i) if i > start_index => file_name.insert_str(i, &inc),
                    _ => file_name.push_str(&inc),
                }
            }
        }

        self.path.set_file_name(file_name);

        Ok(self)
    }

    pub fn finish(self) -> PathBuf {
        self.path
    }
}
