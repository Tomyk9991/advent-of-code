use std::str::FromStr;
use itertools::Itertools;

const A_REG: usize = 0;
const B_REG: usize = 1;
const C_REG: usize = 2;
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Instruction {
    #[default]
    NOOP,
    /*0*/
    ADV(u32), // A    | division with combo operand, result in A
    /*1*/ BXL(u32), // B    | bitwise xor with literal
    /*2*/ BST(u32), // B    | modulo 8 with combo operand
    /*3*/ JNZ(u32), //      | jump to instruction with literal
    /*4*/ BXC(u32), // B, C | bitwise xor
    /*5*/ OUT(u32), //      | combo operand
    /*6*/ BDV(u32), // A    | division with combo operand, result in B
    /*7*/ CDV(u32), // A    | division with combo operand, result in C
}

impl From<Vec<u32>> for Instruction {
    fn from(value: Vec<u32>) -> Self {
        if let [opcode, operand] = value[..] {
            return match (opcode, operand) {
                (0, operand) => Instruction::ADV(operand),
                (1, operand) => Instruction::BXL(operand),
                (2, operand) => Instruction::BST(operand),
                (3, operand) => Instruction::JNZ(operand),
                (4, operand) => Instruction::BXC(operand),
                (5, operand) => Instruction::OUT(operand),
                (6, operand) => Instruction::BDV(operand),
                (7, operand) => Instruction::CDV(operand),
                _ => Instruction::NOOP
            };
        }

        Instruction::NOOP
    }
}

#[derive(Default, Clone, Debug)]
pub struct Day {
    registers: [u32; 3],
    instruction_pointer: usize,
    program: Vec<Instruction>,
    program_literal: Vec<u32>,
}


impl FromStr for Day {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut registers = [0; 3];
        for line in s.lines() {
            let split = &line.split(&[' ', ':']).filter(|a| !a.trim().is_empty()).collect::<Vec<_>>()[..];
            if let ["Register", register, value] = split {
                match *register {
                    "A" => registers[0] = value.parse()?,
                    "B" => registers[1] = value.parse()?,
                    "C" => registers[2] = value.parse()?,
                    _ => {}
                }
            }

            if let ["Program", program] = split {
                let program_values = program.split(',').map(|a| a.parse()).collect::<Result<Vec<_>, _>>()?;
                let program = program_values
                    .iter()
                    .chunks(2)
                    .into_iter()
                    .map(|chunk| Instruction::from(chunk.map(|a| *a).collect::<Vec<_>>()))
                    .collect::<Vec<_>>();

                return Ok(Self {
                    registers,
                    instruction_pointer: 0,
                    program,
                    program_literal: program_values,
                });
            }
        }

        Ok(Day::default())
    }
}

impl crate::aoc::Day for Day {
    type Output = String;

    fn test_cases_1() -> Vec<(&'static str, Self::Output)> {
        vec![(r#"Register A: 0
Register B: 0
Register C: 9

Program: 2,6"#, String::from("")), (r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#, String::from("0,1,2")), (r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#, String::from("4,6,3,5,6,3,5,2,1,0"))]
    }

    fn test_cases_2() -> Vec<(&'static str, Self::Output)> {
        vec![/*(r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#, String::from("117440"))*/]
    }

    fn solution1(&mut self) -> anyhow::Result<Self::Output> {
        let mut output = Vec::new();

        while self.instruction_pointer < self.program.len() {
            let mut ignore_instruction_increment = false;
            let instruction = &self.program[self.instruction_pointer];

            match instruction {
                Instruction::ADV(operand) => self.registers[A_REG] /= 2_u32.pow(to_combo_operand(*operand, &self.registers)),
                Instruction::BXL(operand) => self.registers[B_REG] ^= operand,
                Instruction::BST(operand) => self.registers[B_REG] = to_combo_operand(*operand, &self.registers) % 8,
                Instruction::JNZ(operand) => self.instruction_pointer = if self.registers[A_REG] == 0 { self.instruction_pointer } else {
                    ignore_instruction_increment = true;
                    *operand as usize
                },
                Instruction::BXC(_) => self.registers[B_REG] ^= self.registers[C_REG],
                Instruction::OUT(operand) => output.push(to_combo_operand(*operand, &self.registers) % 8),
                Instruction::BDV(operand) => self.registers[B_REG] = self.registers[A_REG] / 2_u32.pow(to_combo_operand(*operand, &self.registers)),
                Instruction::CDV(operand) => self.registers[C_REG] = self.registers[A_REG] / 2_u32.pow(to_combo_operand(*operand, &self.registers)),
                Instruction::NOOP => {}
            }

            if !ignore_instruction_increment {
                self.instruction_pointer += 1;
            }
        }

        Ok(output.iter().join(","))
    }

    fn solution2(&mut self) -> anyhow::Result<Self::Output> {
        let literal_program = self.program_literal.iter().map(|a| *a as u64).collect::<Vec<_>>();
        let ans = find_solution(&self.program_literal.iter().map(|a| *a as u64).collect::<Vec<_>>(), 0, &literal_program);
        println!("{:?}", ans);
        Ok("".to_string())
    }
}

fn find_solution(program: &[u64], answer: u64, original_program: &Vec<u64>) -> Option<u64> {
    if program.len() == 0 {
        return Some(answer);
    }


    for t in 0..8 {
        let a = answer << 3 | t;
        let mut b = 0;
        let mut c = 0;
        let mut output: Option<u64> = None;

        fn combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
            match operand {
                0..=3 => operand,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!()
            }
        }

        for pointer in (0..original_program.len() - 2).step_by(2) {
            let instruction = original_program[pointer];
            let operand = original_program[pointer + 1];

            match instruction {
                1 => b = b ^ operand,
                2 => b = combo(operand, a, b, c) % 8,
                4 => b = b ^ c,
                3 => panic!(),
                5 => output = Some(combo(operand, a, b, c) % 8),
                6 => b = a >> combo(operand, a, b, c),
                7 => c = a >> combo(operand, a, b, c),
                _ => { }
            }

            if let Some(output) = output {
                if output == program[program.len() - 1] {
                    let sub = find_solution(&program[..program.len() - 1], a, original_program);
                    if sub.is_none() { continue; }
                    return Some(sub.unwrap());
                }
            }
        }
    }

    None
}

fn to_combo_operand(operand: u32, registers: &[u32; 3]) -> u32 {
    match operand {
        a if a >= 0 && a <= 3 => a,
        4 => registers[A_REG],
        5 => registers[B_REG],
        6 => registers[C_REG],
        _ => panic!()
    }
}