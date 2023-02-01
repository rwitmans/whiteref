use core::fmt::Display;
use std::str::FromStr;

use whitespacers::Program;

#[derive(PartialEq, Clone, Debug)]
pub struct Instruction {
    pub command: String,
    pub parameter: Option<String>,
}

impl Instruction {
    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_parameter(&self) -> Option<String> {
        self.parameter.clone()
    }

    pub fn set_parameter(&mut self, parameter: Option<String>) {
        self.parameter = parameter;
    }

    pub fn construct_program_string(self) -> String {
        match self.get_parameter() {
            Some(par) => format!("{} {}\n", self.get_command(), par),
            None => format!("{}\n", self.get_command()),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.parameter {
            Some(parameter) => write!(f, "{} {}", self.command, parameter),
            None => write!(f, "{}", self.command),
        }
    }
}

pub fn _from_string(instruction_string: String) -> Instruction {
    let instruction_split: Vec<String> = instruction_string
        .trim()
        .split(' ')
        .map(String::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    if instruction_split.len() == 1 {
        Instruction {
            command: instruction_string,
            parameter: None,
        }
    } else {
        Instruction {
            command: instruction_split[0].clone(),
            parameter: Some(instruction_split[1].clone()),
        }
    }
}

pub fn from_str(instruction_str: &str) -> Instruction {
    let instruction_split: Vec<String> = instruction_str
        .trim()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(String::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    if instruction_split.len() == 1 {
        Instruction {
            command: instruction_str.trim().to_string(),
            parameter: None,
        }
    } else {
        Instruction {
            command: instruction_split[0].trim().to_string(),
            parameter: Some(instruction_split[1].clone()),
        }
    }
}

pub fn _construct_instruction_set(program: &Program) -> Vec<Instruction> {
    program.disassemble().lines().map(from_str).collect()
}

pub fn construct_program(instructions: Vec<Instruction>) -> Program {
    let instruction_string: String = instructions
        .into_iter()
        .map(|x| x.construct_program_string())
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&x);
            acc
        });

    match Program::assemble(instruction_string) {
        Ok(program) => program,
        Err(e) => panic!("Something internal went wrong: {}", e),
    }
}
