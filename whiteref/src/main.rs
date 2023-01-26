mod actions;
mod instructions;
mod refactorings;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please give the source of the whitespace program and the destination of the refactored whitespace code.");
        process::exit(0);
    }

    let command = args[1].as_str();

    match command {
        "assemble" => actions::assemble_whitespace_file(args),
        "run" => actions::run_whitespace_file(args),
        "refactor" => actions::refactor_whitespace_file(args),
        "dissasemble" => actions::dissasemble_whitespace_file(args),
        _ => println!("Choose a valid command."),
    }
}
