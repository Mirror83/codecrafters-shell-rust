use std::io::{self, Write};

use crate::builtins::Builtin;
mod builtins;

enum CommandType {
    Builtin(builtins::Builtin),
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

enum CommandResultValue {
    Exit,
    Output(String),
}

struct CommandError {
    pub reason: String,
}

type CommandResult = Result<Option<CommandResultValue>, CommandError>;

pub struct Command {
    command_type: CommandType,
    args: Vec<String>,
}

impl Command {
    fn from_string(input: &String) -> Option<Command> {
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

    fn run(&self) -> CommandResult {
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

pub struct Shell {
    prompt_sign: String,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            prompt_sign: "$".to_string(),
        }
    }

    pub fn run(&self) {
        loop {
            let result = match self.read() {
                Some(command) => self.eval(&command),
                None => continue,
            };
            self.print(&result);
            if self.should_exit(&result) {
                break;
            };
        }
    }

    fn read(&self) -> Option<Command> {
        print!("{} ", self.prompt_sign);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        Command::from_string(&input)
    }

    fn eval(&self, command: &Command) -> CommandResult {
        command.run()
    }

    fn print(&self, result: &CommandResult) {
        match result {
            Ok(Some(CommandResultValue::Output(text))) => println!("{}", text),
            Ok(None) | Ok(Some(CommandResultValue::Exit)) => {}
            Err(err) => println!("{}", err.reason),
        }
    }

    fn should_exit(&self, result: &CommandResult) -> bool {
        match result {
            Ok(Some(CommandResultValue::Exit)) => true,
            _ => false,
        }
    }
}
