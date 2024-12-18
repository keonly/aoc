use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy)]
struct Registers {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
}

impl FromStr for Registers {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Register\sA:\s(?P<reg_a>\d+)\nRegister\sB:\s(?P<reg_b>\d+)\nRegister\sC:\s(?P<reg_c>\d+)",
        )
        .map_err(|e| e.to_string())?;

        let caps = re
            .captures(s)
            .ok_or_else(|| format!("Input does not match the expected format: {}", s))?;

        let reg_a: usize = caps["reg_a"]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let reg_b: usize = caps["reg_b"]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let reg_c: usize = caps["reg_c"]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        Ok(Self {
            reg_a,
            reg_b,
            reg_c,
        })
    }
}

type Program = Vec<u8>;

#[derive(Debug, Clone)]
struct Computer {
    regs: Registers,
    program: Program,
    ip: usize,
}

impl FromStr for Computer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut descriptions = s.split("\n\n");
        let reg_str = descriptions
            .next()
            .ok_or_else(|| "Register information required".to_string())?;
        let pgr_str = descriptions
            .next()
            .ok_or_else(|| "Program information required".to_string())?;
        let regs = Registers::from_str(reg_str)?;
        let program = Computer::parse_program(pgr_str)?;
        Ok(Self {
            regs,
            program,
            ip: 0,
        })
    }
}

impl Computer {
    fn parse_program(s: &str) -> Result<Program, String> {
        Ok(s.split(':')
            .nth(1)
            .unwrap_or("")
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .collect())
    }

    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.regs.reg_a,
            5 => self.regs.reg_b,
            6 => self.regs.reg_c,
            _ => panic!("Invalid operand: {}", operand),
        }
    }

    fn execute(&mut self, opcode: u8, operand: u8) -> Option<u8> {
        match opcode {
            0 => {
                // ADV
                self.ip += 2;
                let div_result = self.regs.reg_a as f64
                    / 2_usize.pow(
                        u32::try_from(self.combo_operand(operand))
                            .expect("Should be convertible as u32"),
                    ) as f64;
                self.regs.reg_a = div_result.floor() as usize;
                None
            }
            1 => {
                // BXL
                self.ip += 2;
                self.regs.reg_b ^= operand as usize;
                None
            }
            2 => {
                // BST
                self.ip += 2;
                self.regs.reg_b = self.combo_operand(operand) % 8;
                None
            }
            3 => {
                // JNZ
                if self.regs.reg_a == 0 {
                    self.ip += 2;
                } else {
                    self.ip = operand as usize;
                }
                None
            }
            4 => {
                // BXC
                self.ip += 2;
                self.regs.reg_b ^= self.regs.reg_c;
                None
            }
            5 => {
                // OUT
                self.ip += 2;
                Some(
                    u8::try_from(self.combo_operand(operand) % 8)
                        .expect("Should be convertible as u8"),
                )
            }
            6 => {
                // BDV
                self.ip += 2;
                let div_result = self.regs.reg_a as f64
                    / 2_usize.pow(
                        u32::try_from(self.combo_operand(operand))
                            .expect("Should be convertible as u32"),
                    ) as f64;
                self.regs.reg_b = div_result.floor() as usize;
                None
            }
            7 => {
                // CDV
                self.ip += 2;
                let div_result = self.regs.reg_a as f64
                    / 2_usize.pow(
                        u32::try_from(self.combo_operand(operand))
                            .expect("Should be convertible as u32"),
                    ) as f64;
                self.regs.reg_c = div_result.floor() as usize;
                None
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    fn simulate(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        let program_len = self.program.len();

        while self.ip + 1 < program_len {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            let result = self.execute(opcode, operand);
            if let Some(r) = result {
                output.push(r);
            }
        }

        output
    }
}
fn backtrack(mut computer: Computer) -> Vec<usize> {
    let mut possible_vals: Vec<usize> = vec![];
    let initial_regs = computer.regs;

    let output = computer.simulate();
    if output == computer.program {
        possible_vals.push(initial_regs.reg_a);
        return possible_vals;
    } else if computer.program.ends_with(&output) {
        for i in 0..=7 {
            let new_computer = Computer {
                regs: Registers {
                    reg_a: (initial_regs.reg_a << 3) | i,
                    reg_b: initial_regs.reg_b,
                    reg_c: initial_regs.reg_c,
                },
                program: computer.program.clone(),
                ip: 0,
            };

            possible_vals.extend(backtrack(new_computer));
        }
    }

    possible_vals
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::from_str(input).expect("Failed to parse computer");
    let output = computer
        .simulate()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Some(output)
}

pub fn part_two(input: &str) -> Option<usize> {
    let computer = Computer::from_str(input).expect("Failed to parse computer");
    let initial_regs = computer.regs;
    let mut possible_vals: Vec<usize> = vec![];

    for i in 1..=7 {
        let new_computer = Computer {
            regs: Registers {
                reg_a: i,
                reg_b: initial_regs.reg_b,
                reg_c: initial_regs.reg_c,
            },
            program: computer.program.clone(),
            ip: 0,
        };
        possible_vals.extend(backtrack(new_computer));
    }

    let min_val = possible_vals.iter().min().unwrap();

    Some(*min_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("5,7,3,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
