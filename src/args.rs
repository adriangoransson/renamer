use regex::Regex;
use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

/// Parse a single key-value pair
// https://github.com/TeXitoi/structopt/blob/master/examples/keyvalue.rs
fn parse_pattern(s: &str) -> Result<(Regex, String), Box<dyn Error>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid REGEX=REPLACEMENT: no `=` found in `{}`", s))?;

    let pattern = Regex::new(&s[..pos])?;

    Ok((pattern, s[pos + 1..].parse()?))
}

fn parse_increment(s: &str) -> Result<Increment, Box<dyn Error>> {
    Ok(Increment {
        width: s.len(),
        start: s.parse()?,
    })
}

// TODO: string patterns (like "005-" or "_01") around increment.
#[derive(Debug, Copy, Clone)]
pub struct Increment {
    pub width: usize,
    pub start: usize,
}

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Options {
    /// Test the regular expression against all possible matches instead of only the first.
    #[structopt(short, long)]
    pub global: bool,

    /// Perform a dry-run. Do everything but the actual renaming. Implies verbose.
    #[structopt(short, long)]
    pub dry_run: bool,

    /// Print operations as they are being performed.
    #[structopt(short, long)]
    pub verbose: bool,

    /// Do not exit or ask for confirmation when overwriting files.
    #[structopt(short, long)]
    pub force: bool,

    /// Ask for confirmation before overwrite. The program will otherwise exit unless --force is passed.
    #[structopt(short, long)]
    pub interactive: bool,

    /// Ignores directories passed to the program as files. Useful for shell globbing.
    #[structopt(long)]
    pub ignore_invalid_files: bool,

    /// Prefix files with an increasing counter in the specified format. E.g. 0501 => 0501filename, 0502filename. Applied after pattern replacements.
    #[structopt(long, parse(try_from_str = parse_increment))]
    pub prefix_increment: Option<Increment>,

    /// See --prefix-increment. Will try to insert suffix before the file extension.
    #[structopt(long, parse(try_from_str = parse_increment))]
    pub suffix_increment: Option<Increment>,

    /// Regex pattern to match and the string to replace it with. (REGEX=REPLACEMENT)
    #[structopt(required = true, parse(try_from_str = parse_pattern))]
    pub pattern: (Regex, String),

    /// Additional patterns. These can be supplied multiple times. Patterns are executed in the order they are passed, starting with the mandatory pattern.
    #[structopt(short = "e", long = "regexp", parse(try_from_str = parse_pattern), number_of_values = 1)]
    pub patterns: Vec<(Regex, String)>,

    /// Files to rename.
    #[structopt(required = true)]
    pub files: Vec<PathBuf>,
}
