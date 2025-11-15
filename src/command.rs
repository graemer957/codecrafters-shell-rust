pub enum Command {
    Exit { status_code: i32 },
    Echo { args: Vec<String> },
    Type { target: String },
    External { program: String, args: Vec<String> },
    Noop,
}
