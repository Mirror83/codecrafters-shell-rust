[![progress-banner](https://backend.codecrafters.io/progress/shell/3e8730d2-71f6-4301-92ef-5b33349a1435)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is my solution for the ["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

Currently, the code passes the tests for all of the base stages which outline a simple shell with:

- a REPL (Read-Evaluate-Print Loop)
- three built-in commands (echo, exit and type)
  - `echo` prints out the arguments provided to it, separated by spaces
  - `exit` closes the shell
  - `type` checks whether a command is a built-in command or a command that can be found
    from the directories in the `PATH` environment variable
- the ability to execute programs discoverable through `PATH` and print the outputs from these programs

# Structure

`src/command.rs` - Parses input and then run based with
`src/builtins.rs` - Implements the built-in commands
`src/lib.rs` - Brings in `command` and `built-in` and implements the `REPL` within a `Shell` struct

# External crates

- [is_executable](https://crates.io/crates/is_executable) - For checking whether files in `PATH` directories are executable or not

There are more crates in the `Cargo.toml` file (anyhow, bytes and thiserror),
but these came with the default files and I've not directly used them.
