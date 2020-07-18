use renamer::{args::Increment, file_renamer::*};
use std::path::PathBuf;

#[test]
fn increment() {
    let mut fr = FileRenamer::new(PathBuf::from("/tmp/renamertest/where_am_i"));

    fr.increment(
        IncrementPosition::Prefix,
        Increment { width: 4, start: 0 },
        775,
    )
    .unwrap();

    let result = fr.finish();

    assert_eq!(PathBuf::from("/tmp/renamertest/0775where_am_i"), result);
}

#[test]
/// This is intended behavior. The program will not attempt to deduplicate inputs.
fn increment_twice() {
    let mut fr = FileRenamer::new(PathBuf::from("/tmp/renamertest/some test file.txt"));

    for count in 0..2 {
        fr.increment(
            IncrementPosition::Prefix,
            Increment { width: 4, start: 0 },
            count,
        )
        .unwrap();
    }

    let result = fr.finish();

    assert_eq!(
        PathBuf::from("/tmp/renamertest/00010000some test file.txt"),
        result
    );
}

#[test]
fn increment_multiple() {
    let files: Vec<PathBuf> = vec!["hello.txt".into(), "goodbye.ini".into()];

    let expected: Vec<PathBuf> = vec!["hello12.txt".into(), "goodbye13.ini".into()];

    let mut results = Vec::new();
    for (count, path) in files.iter().enumerate() {
        let mut fr = FileRenamer::new(path);

        fr.increment(
            IncrementPosition::Suffix,
            Increment {
                start: 12,
                width: 2,
            },
            count,
        )
        .unwrap();

        results.push(fr.finish());
    }

    assert_eq!(expected, results);
}

#[test]
fn increment_hidden_with_ext() {
    let files: Vec<PathBuf> = vec![".xinitrc".into(), ".hidden.config".into()];
    let expected: Vec<PathBuf> = vec![".122xinitrc0455".into(), ".123hidden0456.config".into()];

    let mut results = vec![];
    for (count, path) in files.iter().enumerate() {
        let mut fr = FileRenamer::new(path);

        fr.increment(
            IncrementPosition::Prefix,
            Increment {
                start: 122,
                width: 3,
            },
            count,
        )
        .unwrap();

        fr.increment(
            IncrementPosition::Suffix,
            Increment {
                start: 455,
                width: 4,
            },
            count,
        )
        .unwrap();

        results.push(fr.finish());
    }

    assert_eq!(expected, results);
}
