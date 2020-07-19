# Renamer
A bulk renaming tool for files.

## Features

* Rename one or several patterns in your files using the powerful [Rust regex engine](https://crates.io/crates/regex).
* Add an increment as a prefix or suffix to files.

And, uh, it's pretty speedy I guess? I'm hoping it's cross platform too but so far I have only tested it on *nix systems.

## Installation

Have a look at the [releases page](https://github.com/adriangoransson/renamer/releases) for pre-built binaries.

With [Cargo](https://github.com/rust-lang/cargo/).

    $ cargo install renamer

## Usage

    USAGE:
        renamer [FLAGS] [OPTIONS] <pattern> <files>...

    FLAGS:
        -d, --dry-run                 Perform a dry-run. Do everything but the actual renaming
        -f, --force                   Do not exit or ask for confirmation when overwriting files
        -g, --global                  Test the regular expression against all possible matches instead of only the first
        -h, --help                    Prints help information
            --ignore-invalid-files    Ignores directories passed to the program as files. Useful for shell globbing
        -i, --interactive             Ask for confirmation before overwrite. The program will otherwise exit unless --force
                                    is passed
        -V, --version                 Prints version information
        -v, --verbose                 Print operations as they are being performed

    OPTIONS:
        -e, --regexp <patterns>...
                Additional patterns. These can be supplied multiple times. Patterns are executed in the order they are
                passed, starting with the mandatory pattern
            --prefix-increment <prefix-increment>
                Prefix files with an increasing counter in the specified format. E.g. 0501 => 0501filename, 0502filename

            --suffix-increment <suffix-increment>
                See --prefix-increment. Will try to insert suffix before the file extension


    ARGS:
        <pattern>     Regex pattern to match and the string to replace it with. (REGEX=REPLACEMENT)
        <files>...    Files to rename

## Examples

Add a prefix or a file extension.

    # Add a prefix
    $ renamer '^=2020-07-18 ' img*

    # Add an extension
    $ renamer '$=.bak' file1 file2

    # Change extension
    $ renamer 'JPEG$=jpg' *.JPEG

    # Multiple patterns. Change extension and remove a prefix.
    $ renamer 'JPEG$=jpg' -e '^some_prefix_=' *

Rearrange parts of files. The following describes the various ways to use capture groups, including named groups.

    $ renamer --verbose '(?P<index>\d{2}\.) (.*)\.(?P<ext>)=${index} Lady Gaga - $2.$ext' *.mp3

    "01. Chromatica I.mp3" -> "01. Lady Gaga - Chromatica I.mp3"
    "02. Alice.mp3" -> "02. Lady Gaga - Alice.mp3"
    "03. Stupid Love.mp3" -> "03. Lady Gaga - Stupid Love.mp3"
    "04. Rain On Me.mp3" -> "04. Lady Gaga - Rain On Me.mp3"
    "05. Free Woman.mp3" -> "05. Lady Gaga - Free Woman.mp3"
    "06. Fun Tonight.mp3" -> "06. Lady Gaga - Fun Tonight.mp3"
    "07. Chromatica II.mp3" -> "07. Lady Gaga - Chromatica II.mp3"
    "08. 911.mp3" -> "08. Lady Gaga - 911.mp3"
    "09. Plastic Doll.mp3" -> "09. Lady Gaga - Plastic Doll.mp3"
    "10. Sour Candy.mp3" -> "10. Lady Gaga - Sour Candy.mp3"
    "11. Enigma.mp3" -> "11. Lady Gaga - Enigma.mp3"
    "12. Replay.mp3" -> "12. Lady Gaga - Replay.mp3"
    "13. Chromatica III.mp3" -> "13. Lady Gaga - Chromatica III.mp3"
    "14. Sine From Above.mp3" -> "14. Lady Gaga - Sine From Above.mp3"
    "15. 1000 Doves.mp3" -> "15. Lady Gaga - 1000 Doves.mp3"

Add digits to easily sort files. Useful if you were to flatten directory structures but still want your files nicely sorted.

    $ renamer -v '^=_' --prefix-increment 0201 Westworld01.mkv Westworld.S02E02.mkv Westworld_3.mkv

    "Westworld01.mkv" -> "0201_Westworld01.mkv"
    "Westworld.S02E02.mkv" -> "0202_Westworld.S02E02.mkv"
    "Westworld_3.mkv" -> "0203_Westworld_3.mkv"

Also possible to add suffixes with `--prefix-suffix`.

## Acknowledgements
Inspired greatly by the original [`rename.pl`](https://metacpan.org/source/PEDERST/rename-1.9/README.md). The aim is to have similar features but with faster execution time and a slightly more intuitive syntax for those not so familiar with regexes.
