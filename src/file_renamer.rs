use crate::args::Increment;
use regex::Regex;
use std::path::{Path, PathBuf};

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

    pub fn apply_patterns(&mut self, replace_all: bool, patterns: &[(Regex, String)]) -> &mut Self {
        let replace = if replace_all {
            Regex::replace_all
        } else {
            Regex::replace
        };

        // FIXME: holy guacamole.
        let mut file_name = self.path.file_name().unwrap().to_str().unwrap().to_string();

        for (regex, replacement) in patterns {
            let rep = replacement.as_str();
            file_name = replace(regex, &file_name, rep).to_string();
        }

        self.path.set_file_name(file_name);

        self
    }

    pub fn increment(
        &mut self,
        position: IncrementPosition,
        increment: Increment,
        count: usize,
    ) -> &mut Self {
        // FIXME: bleh.
        let mut file_name = self.path.file_name().unwrap().to_str().unwrap().to_string();

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

        self
    }

    pub fn finish(self) -> PathBuf {
        self.path
    }
}
