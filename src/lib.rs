pub mod args;
pub mod errors;
pub mod file_renamer;

mod term_utils;

use errors::{InputError, RenameError};
use file_renamer::{FileRenamer, IncrementPosition};
use std::collections::HashSet;
use term_utils::{ask_for_confirmation, log};

pub fn run(opts: args::Options) -> Result<(), RenameError> {
    if opts.force && opts.interactive {
        return Err(RenameError::InputError(InputError::ForceAndInteractive));
    }

    let verbose = opts.verbose || opts.dry_run;

    // Collect all patterns. The mandatory first and extras in order of input.
    let patterns = {
        let mut p = Vec::with_capacity(1 + opts.patterns.len());

        p.push(opts.pattern);
        p.extend(opts.patterns);

        p
    };

    // Dry run: keep track of unavailable file names.
    let mut paths = HashSet::new();

    // The counter used for increment operations. Incremented for every iteration where a (dry) rename happened.
    let mut count = 0;

    for path in &opts.files {
        if path.is_file() {
            // Apply all renaming operations using a builder.
            let renamed = {
                let mut r = FileRenamer::new(path);

                r.apply_patterns(opts.global, &patterns)?;

                if let Some(prefix_increment) = opts.prefix_increment {
                    r.increment(IncrementPosition::Prefix, prefix_increment, count)?;
                }

                if let Some(suffix_increment) = opts.suffix_increment {
                    r.increment(IncrementPosition::Suffix, suffix_increment, count)?;
                }

                r.finish()
            };

            if path == &renamed {
                if verbose {
                    log(opts.dry_run, format!("No patterns match {:?}", path));
                }

                continue;
            }

            if renamed.is_dir() {
                return Err(RenameError::InputError(
                    InputError::CannotRenameFileToDirectory(path.to_owned(), renamed),
                ));
            }

            if renamed.is_file() || paths.contains(&renamed) {
                if opts.interactive {
                    if !ask_for_confirmation(format!("Overwrite {:?}?", renamed))? {
                        continue;
                    }
                } else if !opts.force {
                    return Err(RenameError::InputError(InputError::SkippingOverwrite(
                        path.to_owned(),
                        renamed,
                    )));
                }
            }

            if verbose {
                log(opts.dry_run, format!("{:?} -> {:?}", path, renamed));
            }

            if opts.dry_run {
                paths.insert(renamed);
            } else {
                std::fs::rename(path, renamed)?;
            }

            count += 1;
        } else if opts.ignore_invalid_files {
            if verbose {
                log(opts.dry_run, format!("Ignoring {:?}", path));
            }
        } else {
            // path is not a file. It might not be a directory either.
            let current = std::env::current_dir()?.join(path);
            return Err(RenameError::InputError(InputError::InvalidFile(current)));
        }
    }

    Ok(())
}
