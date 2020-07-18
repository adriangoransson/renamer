use crate::args::Increment;
use regex::Regex;
use std::path::{Path, PathBuf};

pub struct FileRenamer {
    pub dir: PathBuf,
    pub filename: String,
}

#[derive(Debug)]
pub enum IncrementPosition {
    Prefix,
    Suffix,
}

impl FileRenamer {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = path.as_ref().file_name().unwrap();
        let dir = path.as_ref().parent().unwrap().to_owned();
        let filename = file.to_str().unwrap().to_string();

        FileRenamer { dir, filename }
    }

    pub fn apply_patterns(&mut self, replace_all: bool, patterns: &[(Regex, String)]) -> &mut Self {
        let replace = if replace_all {
            Regex::replace_all
        } else {
            Regex::replace
        };

        for (regex, replacement) in patterns {
            self.filename = replace(regex, &self.filename, replacement.as_str()).to_string();
        }

        self
    }

    pub fn increment(
        &mut self,
        position: IncrementPosition,
        increment: Increment,
        count: usize,
    ) -> &mut Self {
        let inc = format!(
            "{:0width$}",
            increment.start + count,
            width = increment.width
        );
        match position {
            IncrementPosition::Prefix => self.filename.insert_str(0, &inc),
            IncrementPosition::Suffix => {
                if let Some(index) = self.filename.rfind('.') {
                    self.filename.insert_str(index, &inc);
                } else {
                    self.filename.push_str(&inc)
                }
            }
        }

        self
    }

    pub fn finish(self) -> PathBuf {
        self.dir.join(self.filename)
    }
}
