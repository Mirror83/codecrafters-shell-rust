use std::io::{self, Write};

use crate::command::{Command, CommandResult, CommandResultValue};

mod builtins;
mod command;

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
            Ok(Some(CommandResultValue::InPathOutput(output))) => {
                io::stdout()
                    .write_all(&output.stdout)
                    .unwrap_or_else(|err| {
                        eprintln!("Unable to print command stdout output: {:?}", err)
                    });
                io::stderr()
                    .write_all(&output.stderr)
                    .unwrap_or_else(|err| {
                        eprintln!("Unable to print command stderr output: {:?}", err)
                    });
            }
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
