use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use crate::lexer::Lexer;

#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    fn run(&mut self, source: String) {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens();

        match tokens {
            Ok(()) => {
                for token in lexer.tokens {
                    println!("{:?}", token)
                }
            }
            Err(errs) => {
                for (line, message) in errs {
                    self.error(line, &message);
                }

                for token in lexer.tokens {
                    println!("{:?}", token)
                }
            }
        }
    }

    pub fn run_file(&mut self, path: &str) {
        let source = fs::read_to_string(path).unwrap();

        self.run(source);

        if self.had_error {
            exit(65);
        }
    }

    pub fn run_promt(&mut self) {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(input_len) => {
                self.run(input);

                self.had_error = false;

                if input_len > 1 {
                    self.run_promt()
                }
            }
            Err(error) => println!("{error}"),
        }
    }

    pub fn error(&mut self, line: usize, message: &String) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, r#where: &str, message: &str) {
        println!("[line: {line}] Error {where}: {message}");
        self.had_error = true;
    }
}
