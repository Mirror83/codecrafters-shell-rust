use std::io::{self, Write};

enum EvalResultValue {
    Exit,
}

struct EvalError {
    pub reason: String,
}

type EvalResult = Result<Option<EvalResultValue>, EvalError>;

fn read() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().to_string()
}

fn eval(input: &String) -> EvalResult {
    if input == "exit" {
        Ok(Some(EvalResultValue::Exit))
    } else {
        Err(EvalError {
            reason: format!("{}: command not found", input),
        })
    }
}

fn print(result: &EvalResult) {
    match result {
        Ok(_) => {}
        Err(err) => println!("{}", err.reason),
    }
}

fn should_exit(result: &EvalResult) -> bool {
    if let Ok(output) = result {
        if let Some(value) = output {
            match value {
                EvalResultValue::Exit => return true,
            }
        }
    }
    return false;
}

pub fn run() {
    loop {
        let input = read();
        let result = eval(&input);
        print(&result);
        if should_exit(&result) {
            break;
        };
    }
}
