use num_traits::Euclid;
use regex::Regex;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::iter::successors;
use std::ops::BitXor;
use std::sync::LazyLock;
use Instruction::{Bst, Bxc, Bxl, Div, Jnz, Out};

const EXTRACTOR: LazyLock<Regex> = LazyLock::new(|| Regex::new("(\\d+)").unwrap());

pub fn execute(groups: &[&[String]]) -> String {
    let mut registers = get_registers(groups[0]);
    let program = get_program(&groups[1][0]);

    let mut instructor_pointer = 0;
    let mut output = Vec::<usize>::new();

    loop {
        let Some((new_ptr, new_output)) = run(&mut registers, &program, instructor_pointer) else {
            break
        };

        instructor_pointer = new_ptr;
        if let Some(value) = new_output {
            output.push(value)
        }
    }

    output.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(",")
}

pub fn find(groups: &[&[String]]) -> usize {
    let registers = get_registers(groups[0]);
    let program = get_program(&groups[1][0]);

    successors(Some(0usize), |prev| Some(prev + 1)).filter(|a_value| {
        let mut instructor_pointer = 0;
        let mut registers = registers.clone();
        registers[0] = *a_value;

        let mut queue = LinkedList::from_iter(program.iter());

        loop {
            let Some((new_ptr, new_output)) = run(&mut registers, &program, instructor_pointer) else {
                return queue.is_empty()
            };

            instructor_pointer = new_ptr;
            if let Some(value) = new_output {
                if let Some(expected) = queue.pop_front() {
                    if value != *expected {
                        return false
                    }
                } else {
                    return false
                }
            }
        }
    }).nth(0).unwrap()
}

fn run(registers: &mut Vec<usize>, program: &Vec<usize>, instructor_pointer: usize) -> Option<(usize, Option<usize>)> {
    let instruction = Instruction::from(&program[instructor_pointer..])?;
    Some(instruction.apply(registers, instructor_pointer))
}

fn get_registers(lines: &[String]) -> Vec<usize> {
    lines.iter().map(|line| {
        let (_, [num]) = EXTRACTOR.captures(line).unwrap().extract();
        usize::from_str_radix(num, 10).unwrap()
    }).collect::<Vec<_>>()
}

fn get_program(line: &String) -> Vec<usize> {
    EXTRACTOR.captures_iter(line.as_str()).map(|capture| {
        let (_, [num]) = capture.extract();
        usize::from_str_radix(num, 10).unwrap()
    }).collect::<Vec<_>>()
}

#[derive(Debug)]
enum Instruction {
    Div(usize, usize),
    Bxl(usize),
    Bxc,
    Bst(usize),
    Out(usize),
    Jnz(usize),
}

impl Instruction {
    fn from(program: &[usize]) -> Option<Instruction> {
        if program.len() < 2 {
            return None;
        }

        let inst = match program[0] {
            1 => Bxl(program[1]),
            2 => Bst(program[1]),
            3 => Jnz(program[1]),
            4 => Bxc,
            5 => Out(program[1]),
            num => Div(num % 5, program[1])
        };

        Some(inst)
    }

    fn apply(&self, registers: &mut Vec<usize>, pointer: usize) -> (usize, Option<usize>) {
        match self {
            Div(register, combo) => registers[*register] = registers[0] / 2usize.pow(Self::combo_value(registers, *combo) as u32),
            Bxl(literal) => registers[1] = registers[1].bitxor(literal),
            Bst(combo) => registers[1] = Self::combo_value(registers, *combo) % 8,
            Bxc => registers[1] = registers[1].bitxor(registers[2]),
            Out(combo) => return (pointer + 2, Some(Self::combo_value(registers, *combo) % 8)),
            Jnz(literal) => if registers[0] > 0 {
                return (*literal, None)
            }
        }

        (pointer + 2, None)
    }

    fn combo_value(registers: &Vec<usize>, value: usize) -> usize {
        let (data_type, right) = value.div_rem_euclid(&4);
        match data_type {
            0 => right,
            _ => registers[right]
        }
    }
}
