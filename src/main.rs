use std::io::{self, Write};

const BUILTINS: [&str; 1] = ["exit"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let line = input.trim();
                let mut tokens = line.split_whitespace();
                let Some(command) = tokens.next() else {
                    continue;
                };

                if BUILTINS.contains(&command) {
                    if command == "exit" {
                        let status = tokens
                            .next()
                            .map_or(0, |token| token.parse::<i32>().unwrap_or_default());

                        exit(status);
                    }
                } else {
                    println!("{line}: command not found");
                }
            }
            Err(error) => eprintln!("error: {error}"),
        }
    }
}

// TODO: Move to builtin module
fn exit(status: i32) {
    std::process::exit(status);
}
