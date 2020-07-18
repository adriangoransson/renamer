use std::io::stdin;

pub(crate) fn log<S: std::convert::AsRef<str>>(dry_run: bool, message: S) {
    if dry_run {
        print!("DRY ");
    }

    println!("{}", message.as_ref());
}

pub(crate) fn ask_for_confirmation<S: std::convert::AsRef<str>>(
    message: S,
) -> std::io::Result<bool> {
    eprint!("{} [y/N] ", message.as_ref());
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase() == "y")
}
