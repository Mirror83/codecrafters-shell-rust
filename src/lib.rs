use std::io::{self, Write};

enum CommandResultValue {
    Exit,
    Output(String),
}

struct CommandError {
    pub reason: String,
}

type CommandResult = Result<Option<CommandResultValue>, CommandError>;

pub struct Shell {
    prompt_sign: String,
}

pub struct Command {
    name: String,
    args: Vec<String>,
}

impl Command {
    fn from_string(input: &String) -> Option<Command> {
        let input = input.trim();

        let tokens: Vec<String> = input
            .split(" ")
            .map(|token| token.trim().to_string())
            .collect();

        if tokens.len() == 0 {
            return None;
        }

        Some(Command {
            name: tokens[0].to_string(),
            args: tokens[1..].to_vec(),
        })
    }

    fn run(&self) -> CommandResult {
        match self.name.as_str() {
            "exit" => Ok(Some(CommandResultValue::Exit)),
            "echo" => Ok(Some(CommandResultValue::Output(self.args.join(" ")))),
            "type" => {
                println!("type: number of args {}", self.args.len());
                let command_name = match self.args.first() {
                    Some(command_name) => command_name,
                    None => {
                        return Err(CommandError {
                            reason: "find: too many arguments, expected one".to_string(),
                        });
                    }
                };

                match command_name.as_str() {
                    "exit" | "echo" | "type" => Ok(Some(CommandResultValue::Output(format!(
                        "{} is a shell builtin",
                        command_name
                    )))),
                    _ => Err(CommandError {
                        reason: format!("{}: not found", command_name),
                    }),
                }
            }
            other => Err(CommandError {
                reason: format!("{}: command not found", other),
            }),
        }
    }
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
