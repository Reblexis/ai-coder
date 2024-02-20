use std::io::{self, BufRead};

pub fn read_stdin() -> String {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();

    for line in handle.lines() {
        match line {
            Ok(line) => {
                input.push_str(&line);
                input.push('\n'); // Optionally add the newline character back
            },
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
    println!("\n -- Received! -- ");

    input
}