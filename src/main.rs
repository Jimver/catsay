use std::env;
use std::io::{self, Read};

const LINE_WIDTH: usize = 40;

const HELP_MESSAGE: &str = "catsay v0.1 - This is supposed to help";

const CAT_ASCII_ART: &str = r##"  \                              _
   \                            | \
    \                           | |
     \                          | |
      \    |\                   | |
       \  /, ~\                / /
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
fn wrap_lines(message: &str, width: usize) -> Vec<&str> {
    if message.len() <= width {
        return vec![message];
    } else {
        let mut lines: Vec<&str> = vec![];
        let mut cur_pos: usize = width;

        // Search for first whitespace from width to the left
        while message.chars().nth(cur_pos).unwrap() != ' ' && cur_pos > 0 {
            cur_pos -= 1;
        }

        // If no space was found just split at width length
        if cur_pos == 0 {
            cur_pos = width;
        } // Else increment index to remove space
        else {
            cur_pos += 1;
        }

        // Push the line into the lines array
        let (first, last) = message.split_at(cur_pos);

        lines.push(&first);

        // Recurse into the rest of the message
        let rest= wrap_lines(&last, width);

        // Add recursed lines the line we already have
        lines.extend(&rest);

        return lines;
    }
}

/// Converts an array of lines into a textbox with edges
fn to_text_box(lines: Vec<&str>, width: usize) -> String {

    let mut res= String::from("");
    // In case there is no lines
    if lines.len() == 0 {
        res = String::from(" __ \n<  >\n -- \n");
    } // If there is one line
    else if lines.len() == 1 {
        // Top and bottom borders
        let top_text_box = format!(" {:_^1$} ", "", lines[0].len() + 2);
        let bottom_text_box = format!(" {:-^1$} ", "", lines[0].len() + 2);
        res = format!("{top}\n< {line} >\n{bottom}", top = top_text_box, line = lines[0], bottom = bottom_text_box);
    } // If there is 2 or more lines
    else if lines.len() >= 2 {
        // Top and bottom borders
        let top_text_box = format!(" {:_^1$} ", "", width + 2);
        let bottom_text_box = format!(" {:-^1$} ", "", width + 2);

        // Top and bottom text line
        let beneath_top = format!("/ {: <1$} \\", &lines[0], width);
        let above_bottom = format!("\\ {: <1$} /", &lines[lines.len() - 1], width);

        // Process middle lines
        let middle_lines = &lines[1..lines.len() - 1];
        let mut between: String = String::from("");
        if middle_lines.len() > 1 {
            for middle_line in middle_lines {
                between.push_str(&format!("| {: <1$} |\n", middle_line, width));
            }
        }
        res = format!("{top}\n{beneath}\n{middle}{above}\n{bottom}", top=top_text_box, beneath=beneath_top, middle=between, above=above_bottom, bottom=bottom_text_box);
    }
    return res;
}

/// Print the message with the cat saying it
fn catsays(message: &str) {
    let lines = wrap_lines(message, LINE_WIDTH);
    let text_box = to_text_box(lines, LINE_WIDTH);
    println!("{text}\n{cat}", text=text_box, cat=CAT_ASCII_ART);
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