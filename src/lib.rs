pub mod args;
pub mod file_renamer;

mod term_utils;

use file_renamer::{FileRenamer, IncrementPosition};
use term_utils::{ask_for_confirmation, log};

pub fn run(opts: args::Options) -> Result<(), String> {
    if opts.force && opts.interactive {
        return Err(
            "Received --force and --interactive. Not sure how to continue. Exiting.".to_string(),
        );
    }

    let patterns = {
        let mut p = Vec::with_capacity(1 + opts.patterns.len());

        p.push(opts.pattern);
        p.extend(opts.patterns);

        p
    };

    let mut count = 0;
    for path in &opts.files {
        if path.is_file() {
            let renamed = {
                let mut r = FileRenamer::new(path);

                r.apply_patterns(opts.global, &patterns);

                if let Some(prefix_increment) = opts.prefix_increment {
                    r.increment(IncrementPosition::Prefix, prefix_increment, count);
                }

                if let Some(suffix_increment) = opts.suffix_increment {
                    r.increment(IncrementPosition::Suffix, suffix_increment, count);
                }

                r.finish()
            };

            if path == &renamed {
                if opts.verbose {
                    log(opts.dry_run, format!("No patterns match {:?}", path));
                }
                continue;
            }

            if renamed.is_dir() {
                return Err(format!(
                    "Cannot rename {:?}. {:?} is already a directory.",
                    path, renamed
                ));
            }

            if renamed.is_file() {
                if opts.interactive {
                    if !ask_for_confirmation(format!("Overwrite {:?}?", renamed)) {
                        continue;
                    }
                } else if !opts.force {
                    return Err(format!(
                        "Not overwriting {:?} without --interactive or --force",
                        renamed,
                    ));
                }
            }

            if opts.verbose || opts.dry_run {
                log(opts.dry_run, format!("{:?} -> {:?}", path, renamed));
            }

            if !opts.dry_run {
                std::fs::rename(path, renamed).expect("Failed to rename file");
            }

            count += 1;
        } else if opts.ignore_dir {
            if opts.verbose {
                log(opts.dry_run, format!("Ignoring directory {:?}", path));
            }
        } else {
            let current = std::env::current_dir().unwrap().join(path);
            return Err(format!(
                "{:?} is not a file. If this is intentional, pass --ignore-dir.",
                current
            ));
        }
    }

    Ok(())
}
