use std::env;
use std::io::{self, Read};

const HELP_MESSAGE: &str = "catsay v0.1";

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        process_arguments(args);
        Ok(())
    } else {
        process_stdin()
    }
}

/// Print the message with the cat saying it
fn catsays(message: &str) {
    println!("Cat says: {}", message);
}

/// Processes the argument input
fn process_arguments(args: Vec<String>) {
    if args[1] == "-h" || args[1] == "--help" {
        println!("{}", HELP_MESSAGE);
    } else {
        catsays(&args[1..args.len()].join(" "));
    }
}

/// Processes the stdin input
fn process_stdin() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    catsays(&buffer);
    Ok(())
}