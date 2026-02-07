use std::io::{self, Write};

enum EvalResultValue {
    Exit,
}

struct EvalError {
    pub reason: String,
}

type EvalResult = Result<Option<EvalResultValue>, EvalError>;

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
            let input = self.read();
            let result = self.eval(&input);
            self.print(&result);
            if self.should_exit(&result) {
                break;
            };
        }
    }

    fn read(&self) -> String {
        print!("{} ", self.prompt_sign);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        input.trim().to_string()
    }

    fn eval(&self, input: &String) -> EvalResult {
        if input == "exit" {
            Ok(Some(EvalResultValue::Exit))
        } else {
            Err(EvalError {
                reason: format!("{}: command not found", input),
            })
        }
    }

    fn print(&self, result: &EvalResult) {
        match result {
            Ok(_) => {}
            Err(err) => println!("{}", err.reason),
        }
    }

    fn should_exit(&self, result: &EvalResult) -> bool {
        if let Ok(output) = result {
            if let Some(value) = output {
                match value {
                    EvalResultValue::Exit => return true,
                }
            }
        }
        return false;
    }
}
