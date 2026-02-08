use crate::command::{CommandError, CommandResult, CommandResultValue, CommandType};

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

    match CommandType::from_name(command_name) {
        CommandType::Builtin(_) => Ok(Some(CommandResultValue::Output(format!(
            "{} is a shell builtin",
            command_name
        )))),
        CommandType::InPath(_, path) => Ok(Some(CommandResultValue::Output(format!(
            "{} is {}",
            command_name,
            path.to_str().unwrap()
        )))),
        CommandType::Invalid(_) => Err(CommandError {
            reason: format!("{}: not found", command_name),
        }),
    }
}
