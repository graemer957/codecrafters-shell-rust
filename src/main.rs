use std::{
    fmt::Display,
    io::{self, Write},
};

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

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
                    match command {
                        "exit" => {
                            let status = tokens
                                .next()
                                .map_or(0, |token| token.parse::<i32>().unwrap_or_default());

                            exit(status);
                        }
                        "echo" => {
                            echo(tokens);
                        }
                        "type" => {
                            let Some(command) = tokens.next() else {
                                continue;
                            };

                            if BUILTINS.contains(&command) {
                                println!("{command} is a shell builtin");
                            } else {
                                println!("{command}: not found");
                            }
                        }
                        // TODO: Parse command into own type?
                        _ => unreachable!("builtin is not being properly handled"),
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

fn echo<T>(iterator: T)
where
    T: Iterator,
    <T as Iterator>::Item: Display,
{
    let mut peekable = iterator.peekable();

    while let Some(word) = peekable.next() {
        print!("{word}");

        if peekable.peek().is_some() {
            print!(" ");
        }
    }

    println!();
}
