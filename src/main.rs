use anyhow::{anyhow, Context, Result};
use std::{
    env,
    fmt::Display,
    fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
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
                                match find_executable(command) {
                                    Ok(path) => println!("{command} is {}", path.display()),
                                    Err(_) => println!("{command}: not found"),
                                }
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

// TODO: Move to utility module
fn find_executable(name: &str) -> Result<PathBuf> {
    const EXECUTE_BIT: u32 = 0o111;

    let path_env = env::var_os("PATH").context("PATH environment variable not set")?;
    for mut path in env::split_paths(&path_env) {
        path.push(name);
        let Ok(metadata) = fs::metadata(&path) else {
            continue;
        };
        if metadata.is_file() && metadata.permissions().mode() & EXECUTE_BIT != 0 {
            return Ok(path);
        }
    }

    Err(anyhow!("{name}: executable not found in PATH"))
}
