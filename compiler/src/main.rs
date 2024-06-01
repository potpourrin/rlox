use std::{
    env::{self},
    process::exit,
};

use crate::lox::Lox;

mod lexer;
mod lox;

fn main() {
    let args = env::args();

    // if args len equal to 1 that means that there are no argument have been provided
    // if args len is equal to 2 there should be a file
    // if args len is more than 2 there should be input for the promt
    let mut lox = Lox::default();
    if args.len() == 1 {
        lox.run_promt();
    } else if args.len() == 2 {
        let args: Vec<String> = env::args().into_iter().collect();
        lox.run_file(&args[0]);
    } else if args.len() > 2 {
        println!("Usage: rlox [script]")
    }
}
