pub enum Command {
    /// Exit the shell with `code`
    Exit { code: i32 },

    /// Echo `args` back to stdout
    Echo { args: Vec<String> },

    /// Ascertain type of `target` (builtin or path to executable)
    Type { target: String },

    /// Run an external executable
    External { program: String, args: Vec<String> },

    /// No command to run
    ///
    /// This could happen because the user just pressed return, or recoverable error occurred and
    /// the user was already notified.
    Noop,
}
