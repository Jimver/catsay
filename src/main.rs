use std::env;
use std::io::{self, Read};

const HELP_MESSAGE: &str = "catsay v0.1 - This is supposed to help";

const CAT_ASCII_ART: &str = r##"   \                             _
    \                           | \
     \                          | |
      \                         | |
       \   |\                   | |
        \ /, ~\                / /
         X     `-.....-------./ /
          ~-. ~  ~              |
             \             /    |
              \  /_     ___\   /
              | /\ ~~~~~   \ |
              | | \        || |
              | |\ \       || )
             (_/ (_/      ((_/"##;

fn main() -> Result<(), std::io::Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        process_arguments(args);
        Ok(())
    } else {
        process_stdin()
    }
}

/// Wraps the message into lines of certain width using a greedy algorithm
fn wrap_lines(message: &str, width: int) -> Vec<&str> {
    if message.len() <= width {
        return vec![message];
    } else {
        let mut lines: Vec<&str> = vec![];
        let mut cur_pos: int = width;

        // Search for first whitespace from width to the left
        while message[cur_pos] != " " {
            cur_pos -= 1;
        }

        lines.push(&message[0..cur_pos]);

        let mut rest: Vec<&str> = wrap_lines(&message[cur_pos..message.len()], width);

        // TODO
        lines.append(&rest);

        return lines;
    }
}

/// Print the message with the cat saying it
fn catsays(message: &str) {
    println!("{text}\n{cat}", text=message, cat=CAT_ASCII_ART);
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
    catsays(&buffer.trim());
    Ok(())
}