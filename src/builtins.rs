use std::fs::DirEntry;

use crate::command::{CommandError, CommandResult, CommandResultValue};
use is_executable::is_executable;

pub enum Builtin {
    Echo,
    Exit,
    Type,
}

pub fn get_builtin(command_name: &str) -> Option<Builtin> {
    match command_name {
        "echo" => Some(Builtin::Echo),
        "exit" => Some(Builtin::Exit),
        "type" => Some(Builtin::Type),
        _ => None,
    }
}

pub fn exit() -> CommandResult {
    Ok(Some(CommandResultValue::Exit))
}

pub fn echo(args: &Vec<String>) -> CommandResult {
    Ok(Some(CommandResultValue::Output(args.join(" "))))
}

pub fn print_type(args: &Vec<String>) -> CommandResult {
    let Some(command_name) = args.first() else {
        return Err(CommandError {
            reason: "type: missing argument".to_string(),
        });
    };

    match get_builtin(command_name) {
        Some(_) => Ok(Some(CommandResultValue::Output(format!(
            "{} is a shell builtin",
            command_name
        )))),
        None => match search_path(command_name) {
            Some(path) => Ok(Some(CommandResultValue::Output(format!(
                "{} is {}",
                command_name,
                path.to_str().unwrap()
            )))),
            None => Err(CommandError {
                reason: format!("{}: not found", command_name),
            }),
        },
    }
}

fn command_is_in_directory(command_name: &str, entry: &DirEntry) -> bool {
    let entry_path = entry.path();
    let Ok(metadata) = entry.metadata() else {
        return false;
    };
    if metadata.is_file()
        && (entry_path.file_name().unwrap() == command_name
            || entry_path.file_stem().unwrap() == command_name)
        && is_executable(&entry_path)
    {
        return true;
    }

    return false;
}

fn search_path(command_name: &str) -> Option<std::path::PathBuf> {
    let Ok(path) = std::env::var("PATH") else {
        panic!("Could not get PATH environment variable values.")
    };
    for directory in std::env::split_paths(&path) {
        match std::fs::read_dir(&directory) {
            Ok(mut read_dir) => {
                let Some(Ok(command_dir_entry)) =
                    read_dir.find(|entry_result| match entry_result {
                        Ok(entry) => command_is_in_directory(command_name, entry),
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
