pub enum Command {
    Exit { status_code: i32 },
    Echo { args: Vec<String> },
    Type { target: String },
    External { program: String, args: Vec<String> },
    Noop,
}

impl Command {
    #[must_use]
    pub fn is_builtin(command: &str) -> bool {
        // I'm not a fan of duplicating this list with `Parser::parse`, but
        // I cannot think of a better way to do it for now!
        ["echo", "exit", "type"].contains(&command)
    }
}
