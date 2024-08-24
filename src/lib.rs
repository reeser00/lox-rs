use std::io;
use std::fs::File;
use std::io::prelude::*;

pub mod token;
pub mod token_type;
pub mod scanner;

use token::Token;
use scanner::Scanner;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
        }
    }

    pub fn main(&mut self, args: Vec<String>) {
        match args.len() {
            1 => self.run_file(&args[0]).unwrap(),
            0 => self.run_prompt().unwrap(),
            _ => {
                eprintln!("Usage: rlox [script]");
                std::process::exit(1);
            }
        }
    }

    fn run_file(&mut self, file_path: &String) -> std::io::Result<()> {
        let mut file: File = File::open(file_path)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;

        self.run(contents);

        if self.had_error { std::process::exit(65); }

        Ok(())
    }

    fn run_prompt(&mut self) -> std::io::Result<()> {
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(0) => return Ok(()), //end-of-line Control-D 
                Ok(_) => {},
                Err(error) => return Err(error),
            };
            
            //Maybe need to remove this later - removes /n from line.
            buffer = buffer.trim().to_string();
            
            self.run(buffer);
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner: Scanner = Scanner::new(source, self);
        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }

    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, String::new(), message);
    }

    fn report(&mut self, line: usize, location: String, message: String) {
        eprintln!(
            "[line {}] Error {}: {}", line, location, message
        );

        self.had_error = true;
    }
}
