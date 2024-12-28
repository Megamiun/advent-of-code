use itertools::Itertools;
use num_traits::Euclid;
use regex::Regex;
use std::fmt::Debug;
use std::ops::BitXor;
use std::sync::LazyLock;
use Instruction::{Bst, Bxc, Bxl, Div, Jnz, Out};

const EXTRACTOR: LazyLock<Regex> = LazyLock::new(|| Regex::new("(\\d+)").unwrap());

#[allow(dead_code)]
pub fn execute([registers, program]: &[&[String]; 2]) -> String {
    let mut registers = get_registers(registers);
    let program = get_program(&program[0]);

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

#[allow(dead_code)]
pub fn find([_, program]: &[&[String]; 2]) -> usize {
    let program = get_program(&program[0]);

    let instructions = program.chunks(2)
        .filter_map(|inst| Instruction::from(inst))
        .collect_vec();

    find_quine(&instructions, &program.iter().rev().collect_vec(), 1).unwrap()
}

fn find_quine(instructions: &[Instruction], outputs: &[&usize], minimum: usize) -> Option<usize> {
    if outputs.is_empty() {
        return Some(minimum >> 3)
    }

    let expected = outputs[0];
    
    (minimum..minimum + 8).filter_map(|a_reg| {
        let registers = &mut [a_reg, 0, 0];
        
        instructions.iter()
            .filter_map(|instruction| instruction.apply(registers, 0).1)
            .nth(0)
            .filter(|result| result == expected)
            .and_then(|_| find_quine(instructions, &outputs[1..], a_reg << 3))
    }).nth(0)
}

fn run(registers: &mut [usize; 3], program: &[usize], instructor_pointer: usize) -> Option<(usize, Option<usize>)> {
    let instruction = Instruction::from(&program[instructor_pointer..])?;
    Some(instruction.apply(registers, instructor_pointer))
}

fn get_registers(lines: &[String]) -> [usize; 3] {
    lines.iter().map(|line| {
        let (_, [num]) = EXTRACTOR.captures(line).unwrap().extract();
        usize::from_str_radix(num, 10).unwrap()
    }).collect_vec().try_into().unwrap()
}

fn get_program(line: &String) -> Vec<usize> {
    EXTRACTOR.captures_iter(line.as_str()).map(|capture| {
        let (_, [num]) = capture.extract();
        usize::from_str_radix(num, 10).unwrap()
    }).collect_vec()
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

    fn apply(&self, registers: &mut [usize; 3], pointer: usize) -> (usize, Option<usize>) {
        match self {
            Div(index, combo) => registers[*index] = registers[0] / 2usize.pow(Self::combo_value(registers, *combo) as u32),
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

    fn combo_value(registers: &[usize; 3], value: usize) -> usize {
        let (data_type, right) = value.div_rem_euclid(&4);
        match data_type {
            0 => right,
            _ => registers[right]
        }
    }
}
