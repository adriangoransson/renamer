use renamer::{args::Options, run};

#[test]
fn no_force_and_interactive() {
    let result = run(Options {
        global: true,
        dry_run: true,
        verbose: true,
        force: true,
        interactive: true,
        ignore_invalid_files: true,
        prefix_increment: None,
        suffix_increment: None,
        pattern: (regex::Regex::new("").unwrap(), String::new()),
        patterns: vec![],
        files: vec![],
    });

    assert!(matches!(
        result,
        Err(renamer::errors::RenameError::InputError(
            renamer::errors::InputError::ForceAndInteractive
        ))
    ));
}
