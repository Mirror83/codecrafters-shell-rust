use crate::builtins::{self, Builtin};

pub enum CommandResultValue {
    Exit,
    Output(String),
}

pub struct CommandError {
    pub reason: String,
}

pub type CommandResult = Result<Option<CommandResultValue>, CommandError>;

pub enum CommandType {
    Builtin(Builtin),
    Other(String),
}

impl CommandType {
    fn from_name(command_name: &String) -> CommandType {
        match builtins::get_builtin(command_name) {
            Some(builtin) => CommandType::Builtin(builtin),
            None => CommandType::Other(command_name.to_string()),
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
            CommandType::Other(name) => Err(CommandError {
                reason: format!("{}: command not found", name),
            }),
        }
    }
}
