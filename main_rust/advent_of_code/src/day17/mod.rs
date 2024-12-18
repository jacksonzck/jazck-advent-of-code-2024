use core::panic;
use std::ops::{BitXor, Div};
use itertools::Itertools;
use regex::Regex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Registers {
    a: usize, 
    b: usize, 
    c: usize,
    instruction_pointer: usize,
}

impl Registers {
    fn get_operand_value(self, operand: usize) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!(),
            _ => panic!()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2, 
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Instruction {
    fn from_number(instruction: usize) -> Instruction {
        match instruction {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!()
        }
    }
    
    fn from_perform(instruction: usize, registers: &mut Registers, literal_operand: usize, out: &mut Vec<usize>) {
        Instruction::from_number(instruction).perform_instruction(registers, literal_operand, out);
    }

    fn perform_instruction(self, registers: &mut Registers, literal_operand: usize, out: &mut Vec<usize>) {
        //let operand_value = registers.get_operand_value(literal_operand);
        match self {
            Instruction::Adv => {
                let numerator = registers.a;
                let denominator = (2 as usize).pow(registers.get_operand_value(literal_operand) as u32);
                registers.a = numerator.div(denominator);
                registers.instruction_pointer += 2;
            },
            Instruction::Bxl => {
                registers.b = registers.b.bitxor(literal_operand);
                registers.instruction_pointer += 2;
            },
            Instruction::Bst => {
                registers.b = registers.get_operand_value(literal_operand) % 8;
                registers.instruction_pointer += 2;
            },
            Instruction::Jnz => {
                if registers.a == 0 {
                    registers.instruction_pointer += 2;
                } else {
                    registers.instruction_pointer = literal_operand;
                }
            },
            Instruction::Bxc => {
                registers.b = registers.b.bitxor(registers.c);
                registers.instruction_pointer += 2;
            },
            Instruction::Out => {
                out.push(registers.get_operand_value(literal_operand) % 8);
                registers.instruction_pointer += 2;
            },
            Instruction::Bdv => {
                let numerator = registers.a;
                let denominator = (2 as usize).pow(registers.get_operand_value(literal_operand) as u32);
                registers.b = numerator.div(denominator);
                registers.instruction_pointer += 2;
            },
            Instruction::Cdv => {
                let numerator = registers.a;
                let denominator = (2 as usize).pow(registers.get_operand_value(literal_operand) as u32);
                registers.c = numerator.div(denominator);
                registers.instruction_pointer += 2;
            },
        }
    }
}


fn part1solution(input: &str) -> String {
    let parser_regex = Regex::new(r"(?s)Register A: ([0-9]+).*Register B: ([0-9]+).*Register C: ([0-9]+).*Program: ((?:[0-9]\,?)*)").unwrap();
    let binding = parser_regex.captures(input).unwrap().extract::<4>();
    let (a_str, b_str, c_str, program) = binding.1.iter().collect_tuple().unwrap();
    let mut registers = Registers {a: a_str.parse().unwrap(), b: b_str.parse().unwrap(), c: c_str.parse().unwrap(), instruction_pointer: 0};
    let rom: Vec<usize> = program.split(',').map(|i| i.parse::<usize>().unwrap()).collect();
    let mut out: Vec<usize> = vec![];
    loop {
        println!("{:?}", registers);
        let Some(instruction) = rom.get(registers.instruction_pointer) else {
            break
        };
        let Some(literal_operand) = rom.get(registers.instruction_pointer + 1) else {
            panic!("BADDD")
        };
        Instruction::from_number(*instruction).perform_instruction(&mut registers, *literal_operand, &mut out);
    }
    out.iter().join(",")
}
fn part2solution(input: &str) -> usize {
    let parser_regex = Regex::new(r"(?s)Register A: ([0-9]+).*Register B: ([0-9]+).*Register C: ([0-9]+).*Program: ((?:[0-9]\,?)*)").unwrap();
    let binding = parser_regex.captures(input).unwrap().extract::<4>();
    let (_a_str, b_str, c_str, program) = binding.1.iter().collect_tuple().unwrap();
    (0..11744000000).into_par_iter().find_first(|h| {
        let mut registers = Registers {a: *h, b: b_str.parse().unwrap(), c: c_str.parse().unwrap(), instruction_pointer: 0};
        let rom: Vec<usize> = program.split(',').map(|i| i.parse::<usize>().unwrap()).collect();
        let mut out: Vec<usize> = vec![];
        loop {
            //println!("{:?}", registers);
            let Some(instruction) = rom.get(registers.instruction_pointer) else {
                break
            };
            let Some(literal_operand) = rom.get(registers.instruction_pointer + 1) else {
                panic!("BADDD")
            };
            Instruction::from_number(*instruction).perform_instruction(&mut registers, *literal_operand, &mut out);
        }
        out.iter().join(",").as_str() == *program
    }).unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day17::{part1solution, part2solution};

    use super::{Instruction, Registers};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), "4635635210");
    }

    #[test]
    fn part1example2() {
        assert_eq!(part1solution(include_str!("input.example2")), "012");
    }

    #[test]
    fn part1example3() {
        assert_eq!(part1solution(include_str!("input.example3")), "42567777310");
    }

    #[test]
    fn part1example4() {
        assert_eq!(part1solution(include_str!("input.example4")), "2");
    }
    #[test]
    fn part1example5() {
        assert_eq!(part1solution(include_str!("input.example5")), "2");
    }

    #[test]
    fn register_examples() {
        let mut registers = Registers { a: 2, b: 3, c: 9, instruction_pointer: 1};
        let mut out: Vec<usize> = vec![];
        Instruction::from_perform(2, &mut registers, 6, &mut out);
        assert_eq!(registers.c, 9);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), "94444");
    }
    
    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example6")), 117440);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real")));
        //assert_eq!(part2solution(include_str!("input.real")), 502);
    }
}