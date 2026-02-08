use is_executable;
use std::{
    fs, path,
    process::{self},
};

use crate::builtins::{self, Builtin};

pub enum CommandResultValue {
    Exit,
    InPathOutput(process::Output),
    Output(String),
}

pub struct CommandError {
    pub reason: String,
}

pub type CommandResult = Result<Option<CommandResultValue>, CommandError>;

pub enum CommandType {
    Builtin(Builtin),
    InPath(String, path::PathBuf),
    Invalid(String),
}

impl CommandType {
    pub fn from_name(command_name: &String) -> CommandType {
        match builtins::get_builtin(command_name) {
            Some(builtin) => CommandType::Builtin(builtin),
            None => {
                if let Some(command_path) = Command::search_path(command_name) {
                    return CommandType::InPath(command_name.to_string(), command_path);
                }
                return CommandType::Invalid(command_name.to_string());
            }
        }
    }
}

pub struct Command {
    command_type: CommandType,
    args: Vec<String>,
}

impl Command {
    pub fn from_string(input: &String) -> Option<Command> {
        let input = input.trim();

        let tokens: Vec<String> = input
            .split(" ")
            .map(|token| token.trim().to_string())
            .collect();

        let name = tokens.first()?;

        Some(Command {
            command_type: CommandType::from_name(name),
            args: tokens[1..].to_vec(),
        })
    }

    pub fn run(&self) -> CommandResult {
        match &self.command_type {
            CommandType::Builtin(Builtin::Exit) => builtins::exit(),
            CommandType::Builtin(Builtin::Echo) => builtins::echo(&self.args),
            CommandType::Builtin(Builtin::Type) => builtins::print_type(&self.args),
            CommandType::InPath(name, path) => {
                Command::run_external_command(name, path.to_str().unwrap(), &self.args)
            }
            CommandType::Invalid(name) => Err(CommandError {
                reason: format!("{}: command not found", name),
            }),
        }
    }

    fn command_is_in_directory(command_name: &str, entry: &fs::DirEntry) -> bool {
        let entry_path = entry.path();
        let Ok(metadata) = entry.metadata() else {
            return false;
        };
        if metadata.is_file()
            && (entry_path.file_name().unwrap() == command_name
                || entry_path.file_stem().unwrap() == command_name)
            && is_executable::is_executable(&entry_path)
        {
            return true;
        }

        return false;
    }

    fn search_path(command_name: &str) -> Option<path::PathBuf> {
        let Ok(path) = std::env::var("PATH") else {
            eprintln!("Could not get PATH environment variable values.");
            return None;
        };
        for directory in std::env::split_paths(&path) {
            match std::fs::read_dir(&directory) {
                Ok(mut read_dir) => {
                    let Some(Ok(command_dir_entry)) =
                        read_dir.find(|entry_result| match entry_result {
                            Ok(entry) => Command::command_is_in_directory(command_name, entry),
                            Err(_) => false,
                        })
                    else {
                        continue;
                    };
                    let command_path = command_dir_entry.path();
                    return Some(command_path);
                }
                Err(_) => {
                    continue;
                }
            }
        }
        return None;
    }

    fn run_external_command(name: &str, _path: &str, args: &Vec<String>) -> CommandResult {
        match process::Command::new(name).args(args).output() {
            Ok(output) => Ok(Some(CommandResultValue::InPathOutput(output))),
            Err(err) => Err(CommandError {
                reason: format!("{} failed. {}", name, err.to_string()),
            }),
        }
    }
}
