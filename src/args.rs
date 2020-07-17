use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid REGEX=REPLACEMENT: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

/* TODO:
date/numbering support?
{pre,suf}fix?
*/

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Options {
    /// Test the regular expression against all possible matches instead of only the first.
    #[structopt(short, long)]
    pub global: bool,

    /// Perform a dry-run. Do everything but the actual renaming.
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
    pub ignore_dir: bool,

    /// Regex pattern to match and the string to replace it with. (REGEX=REPLACEMENT)
    #[structopt(required = true, parse(try_from_str = parse_key_val))]
    pub pattern: (String, String),

    /// Additional patterns. These can be supplied multiple times. Patterns are executed in the order they are passed, starting with the mandatory pattern.
    #[structopt(short = "e", long = "regexp", parse(try_from_str = parse_key_val), number_of_values = 1)]
    pub patterns: Vec<(String, String)>,

    /// Files to rename.
    #[structopt(required = true)]
    pub files: Vec<PathBuf>,
}
