use regex::Regex;
use std::{env, fs};

/* TODO:
renamer from to FILES...
-e
--replace-all /g
--dry-run
--verbose
--interactive
*/

fn main() {
    let from = env::args()
        .nth(1)
        .expect("Expected pattern to be the first argument");
    let to = env::args()
        .nth(2)
        .expect("Expected pattern to be the second argument");

    let from_re = Regex::new(&from).unwrap();

    for entry in fs::read_dir(".").unwrap() {
        let dir = entry.unwrap();
        let file = dir.file_name();
        let filename = file.to_str().unwrap();

        let rep = from_re.replace(filename, to.as_str());

        println!("{}", rep);
    }
}
