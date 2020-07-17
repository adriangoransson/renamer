use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Opt {
    /// Test the regular expression against all possible matches instead of only the first.
    #[structopt(short, long)]
    global: bool,

    /// Perform a dry-run. Do everything but the actual renaming.
    #[structopt(short, long)]
    dry_run: bool,

    /// Print operations as they are being performed.
    #[structopt(short, long)]
    verbose: bool,
}
