# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a CodeCrafters challenge implementation for building a POSIX-compliant shell in Rust. The shell should be capable of:
- Interpreting shell commands
- Running external programs
- Implementing builtin commands (cd, pwd, echo, etc.)
- Command parsing and REPL functionality

## Build and Run Commands

**Local development:**
```bash
# Build and run the shell
./your_program.sh

# Build only (release mode)
cargo build --release --target-dir=/tmp/codecrafters-build-shell-rust --manifest-path Cargo.toml

# Run the compiled binary directly
/tmp/codecrafters-build-shell-rust/release/codecrafters-shell
```

**Testing on CodeCrafters:**
```bash
git commit -am "your message"
git push origin master
```
Test output will be streamed to the terminal after pushing.

## Project Structure

- **src/main.rs**: Entry point - orchestrates the REPL loop (prompt → parse → execute)
- **src/prompt.rs**: Displays the shell prompt and flushes stdout
- **src/parser.rs**: Parses input strings into structured `Command` types
- **src/command.rs**: Core data model representing parsed commands (Exit, Echo, Type, External)
- **src/executor.rs**: Executes `Command` objects by delegating to builtins or external programs
- **src/builtins.rs**: Implementations of shell builtin commands (exit, echo)
- **src/utils.rs**: Utility functions for PATH resolution and program execution
- **Cargo.toml**: Dependencies include `anyhow` for error handling
- **your_program.sh**: Script for building and running the shell locally. Uses release mode and outputs to `/tmp/codecrafters-build-shell-rust/`
- **codecrafters.yml**: CodeCrafters configuration. Currently set to `rust-1.88` buildpack with debug logs disabled

## Architecture Notes

The shell implementation should follow a typical REPL (Read-Eval-Print-Loop) architecture:

1. **Input Reading**: Read commands from stdin
2. **Parsing**: Parse input into command tokens (handling quotes, pipes, redirections, etc.)
3. **Execution**: Execute either builtin commands or external programs
4. **Output**: Display results and prompt for next command

Key areas to implement:
- **Builtin Commands**: Implement shell builtins (cd, pwd, echo, type, exit) that cannot be delegated to external programs
- **External Program Execution**: Use Rust's `std::process::Command` to execute external programs found in PATH
- **PATH Resolution**: Search PATH environment variable to locate executables
- **Command Parsing**: Handle quoted strings, escape sequences, and word splitting

## Development Requirements

- Rust version: 1.80+ (CodeCrafters uses 1.88)
- The build uses `--release` mode by default for performance
- Build artifacts are placed in `/tmp/codecrafters-build-shell-rust/` to keep the project directory clean

## Architecture Decisions

**Command Execution (argv[0] handling):**
- External programs are executed using just the filename (not full path) as `argv[0]`
- This is required by CodeCrafters tests and matches standard shell behavior
- The full path is still used for execution, but `Command::new()` receives the basename
- See comment in `utils::run_program` for details

**Error Handling Strategy:**
- Parse and execution errors are caught and printed to stderr
- The shell continues running after errors (doesn't exit)
- Only fatal errors (like broken stdout) cause shell termination
- This matches standard shell behavior

**Module Organization:**
- Public modules: `command`, `executor`, `parser`, `prompt` (the API)
- Private modules: `builtins`, `utils` (implementation details)
- This separation allows for future extensibility and testing

## Code Review Standards

When reviewing Rust code, always check for:

**Idiomatic Rust:**
- Use of proper type annotations (e.g., `-> !` for functions that never return)
- Concise generic bounds (prefer `T::Item` over `<T as Trait>::Item`)
- Appropriate use of `const` (inside functions when scoped appropriately)
- Error handling patterns (prefer `anyhow::Context` and `anyhow!` macro over manual error conversion)
- Unused imports or dead code

**Error Handling:**
- Avoid `unwrap()` where possible - suggest proper error handling
- Check for discarded important values (e.g., `Ok(_)` when the value matters)
- Ensure `Result` and `Option` are handled appropriately

**Performance & Correctness:**
- Constants defined outside loops/iterations
- Proper use of iterators (avoiding unnecessary allocations)
- Correct logic in conditionals (especially bitwise operations)

**Initial Review Completeness:**
- When asked to review code, provide a comprehensive review covering ALL of the above points in the first pass, not just the most obvious issues. Don't wait for follow-up reviews to mention idiomatic improvements.
- Be pedantic

**When Suggesting Refactorings:**
- Start with the **simplest, most direct solution** that addresses the user's stated goals
- If the user has a specific constraint (e.g., "avoid duplication", "keep error messages in one place"), prioritize solutions that meet that constraint
- When multiple approaches exist, present them in order from simplest to most complex
- Avoid over-engineering or suggesting architectural changes unless explicitly needed
- If a borrow checker issue arises, first consider **parameter passing** before more complex solutions like `Option<&mut>`, split functions, or architectural redesigns
