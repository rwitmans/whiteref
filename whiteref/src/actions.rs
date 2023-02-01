use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    process,
};

use whitespacers::{Options, Program, WsError};

use crate::refactorings::perform_refactorings;

pub fn write_ws_file(program: &Program, target: &str) {
    let ws_code = program.dump();

    let mut ws_file = File::create(target).expect("Unsuccesful attempt at creating a file.");

    ws_file
        .write_all(&ws_code)
        .expect("Unable to write in the newly created file.");
}

fn read_ws_file(path: &str) -> Vec<u8> {
    let ws_file = File::open(path).expect("Unable to open file");
    let metadata = fs::metadata(path).expect("unable to read metadata");
    let mut buf_reader = BufReader::new(ws_file);
    let mut buffer = vec![0; metadata.len() as usize];
    buf_reader.read_exact(&mut buffer).unwrap();

    buffer
}

pub fn get_whitespace_program(source: &str) -> Result<Program, WsError> {
    let ws_file = read_ws_file(source);

    Program::parse(ws_file)
}

pub fn run_whitespace_program(program: Program, options: Option<Options>) {
    let mut stdin_reader = BufReader::new(std::io::stdin());

    let mut stdout = std::io::stdout();

    let options = match options {
        Some(options) => options,
        None => whitespacers::Options::empty(),
    };

    let mut jit = whitespacers::Interpreter::new(&program, options, &mut stdin_reader, &mut stdout);

    match jit.interpret_with_simple_state() {
        Ok(()) => (),
        Err(error) => {
            println!("Error while executing program.\n{}", error);
        }
    };
}

pub fn assemble_whitespace_file(args: Vec<String>) {
    if args.len() < 4 {
        println!("Error: Please specify the source of the whitespace assembly and the destination of the transpiled whitespace code.");
        process::exit(0);
    }

    let source = &args[2];
    let dest = &args[3];

    let binding = read_ws_file(source);
    let source_code = match std::str::from_utf8(&binding) {
        Ok(v) => v,
        Err(e) => panic!("Invalid utf-8 sequence: {}", e),
    };

    let output = Program::assemble(source_code.to_string()).unwrap();

    write_ws_file(&output, dest);

    println!("Succesfully assembled a whitespace program from whitespace assembly.");
}

pub fn dissasemble_whitespace_file(args: Vec<String>) {
    if args.len() < 3 {
        println!("Error: Please specify the source of the whitespace code.");
        process::exit(0);
    }

    let source = &args[2];

    let program = match get_whitespace_program(source) {
        Ok(program) => program,
        Err(error) => {
            println!(
                "Error while parsing whitespace code.\n{}\nStopping whiteref execution...",
                error
            );
            process::exit(0);
        }
    };

    let output = Program::disassemble(&program);

    println!("{}", output);

    println!("Succesfully dissasembled whitespace program.");
}

pub fn refactor_whitespace_file(args: Vec<String>) {
    if args.len() < 4 {
        println!("Error: Please specify the source of the file you want to refactor and the destination of the refactored whitespace code.");
        process::exit(0);
    }

    let source = &args[2];
    let target = &args[3];

    let mut program = match get_whitespace_program(source) {
        Ok(program) => program,
        Err(error) => {
            println!(
                "Error while parsing whitespace code.\n{}\nStopping whiteref execution...",
                error
            );
            process::exit(0);
        }
    };

    program = perform_refactorings(program);

    program.minify();

    write_ws_file(&program, target);

    println!("Creating the new whitespace file has been succesful.");
}

pub fn run_whitespace_file(args: Vec<String>) {
    let source = &args[2];

    let program = match get_whitespace_program(source) {
        Ok(program) => program,
        Err(error) => {
            println!(
                "Error while parsing whitespace code.\n{}\nStopping whiteref execution...",
                error
            );
            process::exit(0);
        }
    };

    run_whitespace_program(program, None);

    println!("Succesfully ran the whitespace file.");
}
