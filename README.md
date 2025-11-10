[![progress-banner](https://backend.codecrafters.io/progress/shell/8464049a-6e17-42e9-aff8-bd6964646199)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your `shell` implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git commit -am "pass 1st stage" # any msg
git push origin master
```

Time to move on to the next stage!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.87)` installed locally
1. Run `./your_program.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# TODOs

This is a collection of TODOs of possible improvements/refactors that I feel
would make this project more elegant / reusable / etc, but do not have the time
for right now:

- [ ] Add unit and/or integration tests
- [ ] Re-add "cargo" lints and fix all concerns
- [ ] CI pipeline on GitHub Actions
- [ ] Eliminate builtin list duplication between `Command::is_builtin` and
  parser match
- [ ] Track exit status (`$?`) for last executed command
- [ ] 'Really good' error messages, like the Rust compiler (I think
  the crate is called `eyre`)
  - **NOTE**: This may not be possible / easy because CodeCrafters expects
    output in a certain way
- [ ] No attempt has been made to make this shell work on anything other than Linux
  - See [quispejo's
    solution](https://github.com/cc-code-examples/curious-gorilla-226481/blob/main/src/main.rs)
    for example of finding an executable on Windows
- [ ] Investigate optimisation of zero-copy command parsing with lifetimes
