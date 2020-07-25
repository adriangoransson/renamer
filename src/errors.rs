use std::{fmt::Display, io, path::PathBuf};

#[derive(Debug)]
pub enum RenameError {
    /// Errors originated by user input.
    InputError(InputError),

    /// General IO errors.
    Io(io::Error),
}

#[derive(Debug)]
pub enum InputError {
    /// Received --force and --interactive. Not sure how to continue.
    ForceAndInteractive,

    /// Cannot rename `file`. `directory` is already a directory.
    CannotRenameFileToDirectory(PathBuf, PathBuf),

    /// `file`. Not overwriting `file` without --interactive or --force.
    SkippingOverwrite(PathBuf, PathBuf),

    /// `path` is not a file. If this is intentional, pass --ignore-invalid-files.
    InvalidFile(PathBuf),

    /// Invalid rename. `file` can't be renamed to `file`.
    InvalidRename(PathBuf, PathBuf),
}

impl Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenameError::InputError(err) => {
                let out = match err {
                    InputError::ForceAndInteractive => {
                        "Received --force and --interactive. Not sure how to continue.".to_string()
                    }
                    InputError::CannotRenameFileToDirectory(file, dir) => format!(
                        "Cannot rename {:?}. {:?} is already a directory.",
                        file, dir
                    ),
                    InputError::SkippingOverwrite(file, renamed) => format!(
                        "{:?}. Not overwriting {:?} without --interactive or --force.",
                        file, renamed,
                    ),
                    InputError::InvalidFile(path) => format!(
                        "{:?} is not a file. If this is intentional, pass --ignore-invalid-files.",
                        path
                    ),
                    InputError::InvalidRename(path, renamed) => format!(
                        "Invalid rename. {:?} can't be renamed to {:?}.",
                        path, renamed
                    ),
                };

                writeln!(f, "{}", out)
            }
            RenameError::Io(err) => writeln!(f, "IO error {}", err),
        }
    }
}

impl From<io::Error> for RenameError {
    fn from(e: io::Error) -> Self {
        RenameError::Io(e)
    }
}
