use crate::instructions;

use instructions::Instruction;
use whitespacers::Program;

use std::iter::Iterator;
use std::str::FromStr;

use itertools::Itertools;

#[derive(PartialEq, Debug, Clone)]
pub struct Method {
    label: (String, u32),
    instructions: Vec<Instruction>,
}

impl Method {
    pub fn convert_method_to_instructions(mut self) -> Vec<Instruction> {
        match self.label.0.as_str() {
            "main" => self.instructions,
            _ => {
                let label_instruction = Instruction {
                    command: format!("_{}:", self.label.0),
                    parameter: None,
                };
                self.instructions.insert(0, label_instruction);
                self.instructions
            }
        }
    }

    pub fn get_label(&self) -> String {
        format!("_{}", self.label.0)
    }

    pub fn modify_label_jumps(
        &mut self,
        removed_labels: Vec<String>,
        grouped_methods: Vec<Vec<Method>>,
    ) -> () {
        self.instructions = self
            .instructions
            .clone()
            .into_iter()
            .map(|mut x| match x.get_command().as_str() {
                "call" | "jmp" | "jz" | "jn" => {
                    if removed_labels.contains(&x.get_parameter().unwrap()) {
                        x.set_parameter(Some(format!(
                            "_{}",
                            get_replacement_method(&grouped_methods, x.clone())
                                .label
                                .0
                                .clone(),
                        )));
                    }
                    x
                }
                _ => x,
            })
            .collect();
    }
}

pub fn perform_refactorings(program: Program) -> Program {
    let mut instructions = instructions::_construct_instruction_set(&program);

    instructions = refactor_unused_label(instructions);
    instructions = refactor_double_method(instructions);

    instructions::construct_program(instructions)
}

pub fn get_replacement_method(methods: &Vec<Vec<Method>>, instruction: Instruction) -> Method {
    methods
        .into_iter()
        .filter(|x| {
            let list: Vec<Method> = x
                .clone()
                .into_iter()
                .filter(|y| y.get_label() == instruction.get_parameter().unwrap())
                .map(|x| x.clone())
                .collect();
            !list.is_empty()
        })
        .map(|x| x.get(0).unwrap())
        .next()
        .unwrap()
        .clone()
}

pub fn get_methods_from_instructions(instructions: &Vec<Instruction>) -> Vec<Method> {
    let mut methods: Vec<Method> = Vec::new();
    let mut label = (String::from_str("main").unwrap(), 0);
    let mut method_instructions: Vec<Instruction> = Vec::new();

    for (index, instruction) in instructions.clone().into_iter().enumerate() {
        if instruction.get_command().chars().next().unwrap() == '_' {
            methods.push(Method {
                label: label.clone(),
                instructions: method_instructions.clone(),
            });

            label = (
                instruction
                    .get_command()
                    .chars()
                    .skip(1)
                    .take(instruction.get_command().len() - 2)
                    .collect::<String>(),
                index as u32,
            );
            method_instructions = Vec::new();
        } else {
            method_instructions.push(instruction);
        }
    }

    methods.push(Method {
        label: label.clone(),
        instructions: method_instructions.clone(),
    });

    methods
}

pub fn convert_methods_to_instructions(methods: Vec<Method>) -> Vec<Instruction> {
    methods
        .into_iter()
        .map(|x| x.convert_method_to_instructions())
        .flatten()
        .collect()
}

pub fn refactor_double_method(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let methods = get_methods_from_instructions(&instructions);

    let grouped_methods: Vec<Vec<Method>> = methods
        .clone()
        .into_iter()
        .group_by(|s| s.instructions.clone())
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<Method>>())
        .collect();

    let mut kept_methods: Vec<Method> = grouped_methods
        .clone()
        .into_iter()
        .map(|x| x.get(0).unwrap().clone())
        .collect();

    let removed_labels: Vec<String> = grouped_methods
        .clone()
        .into_iter()
        .flatten()
        .filter(|x| !kept_methods.contains(x))
        .map(|x| x.get_label())
        .collect();

    kept_methods = kept_methods
        .into_iter()
        .map(|mut x| {
            x.modify_label_jumps(removed_labels.clone(), grouped_methods.clone());
            x
        })
        .collect();
    convert_methods_to_instructions(kept_methods)
}

pub fn refactor_unused_label(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut methods = get_methods_from_instructions(&instructions);

    let mut used_functions: Vec<String> =
        instructions.into_iter().fold(Vec::new(), |mut acc, x| {
            match x.get_command().as_str() {
                "call" | "jmp" | "jz" | "jn" => acc.push(x.get_parameter().clone().unwrap()),
                _ => (),
            }

            acc
        });

    used_functions = used_functions
        .into_iter()
        .map(|mut x| {
            x.remove(0);
            x
        })
        .collect();

    used_functions.push("main".to_string());

    used_functions.sort();
    used_functions.dedup();

    methods = methods
        .into_iter()
        .filter(|x| used_functions.contains(&x.label.0))
        .collect();

    convert_methods_to_instructions(methods)
}
