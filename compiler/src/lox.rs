use std::{
    fs,
    io::{self, Error, Write},
    process::exit,
};

use crate::lexer;

#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    fn run(&mut self, source: &str) {
        let tokens = lexer::scan_tokens(source);

        match tokens {
            Ok(tokens) => {
                dbg!(tokens);
            }
            Err(errors) => {
                self.had_error = true;
                dbg!(errors);
            }
        };
    }

    pub fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let source = fs::read_to_string(path)?;

        self.run(&source[0..source.len()]);

        Ok(())
    }

    pub fn run_promt(&mut self) {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(input_len) => {
                self.run(&input);

                self.had_error = false;

                if input_len > 1 {
                    self.run_promt();
                }
            }
            Err(error) => println!("Line is not a valid UTF-8: {error}"),
        }
    }
}
