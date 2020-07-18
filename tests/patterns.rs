use regex::Regex;
use renamer::file_renamer::*;
use std::path::PathBuf;

fn test_helper(initial: &str, replace_all: bool, patterns: Vec<(&str, &str)>) -> PathBuf {
    let mut fr = FileRenamer::new(PathBuf::from(initial));

    let transformed_patterns = patterns
        .into_iter()
        .map(|(k, v)| (Regex::new(k).unwrap(), v.to_owned()))
        .collect::<Vec<_>>();

    fr.apply_patterns(replace_all, &transformed_patterns)
        .unwrap();

    fr.finish()
}

#[test]
fn single_pattern() {
    assert_eq!(
        PathBuf::from("/tmp/SOme sort of file"),
        test_helper("/tmp/Some sort of file", false, vec![("o", "O")])
    );
}

#[test]
fn single_pattern_replace_all() {
    assert_eq!(
        PathBuf::from("wobdobdoo.yob"),
        test_helper("wabbadabbadoo.yabba", true, vec![("abba", "ob")])
    );
}

#[test]
fn add_prefix() {
    assert_eq!(
        PathBuf::from("Beginning of string-some string"),
        test_helper("some string", false, vec![("^", "Beginning of string-")])
    );
}

#[test]
fn rearrange() {
    assert_eq!(
        PathBuf::from("Song Artist.mp3"),
        test_helper(
            "Artist_Song.mp3",
            false,
            vec![("(Artist)_(?P<named>Song)", "${named} $1")]
        )
    );
}

#[test]
fn multiple_patterns() {
    assert_eq!(
        PathBuf::from("Song Artist.aac"),
        test_helper(
            "Artist_Song.mp3",
            false,
            vec![("(Artist)_(?P<named>Song)", "${named} $1"), ("mp3", "aac")]
        )
    );
}
