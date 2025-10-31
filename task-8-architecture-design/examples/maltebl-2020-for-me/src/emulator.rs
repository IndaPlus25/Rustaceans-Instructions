/*******************************************************
Emulator for the Y(eet)L(mao)69-series chipset
                    DO NOT STEAL
Unauthorized use of YL69 or it's connected technologies
is striclty prohibited. © HAHAFUNNYGOBRRR 2020

Optimised for FOR ME 👉👈 programming language
********************************************************/

use crate::language_specs::*;
use std::{fs, io::BufRead, num::ParseIntError};

pub fn execute(filepath: String) -> Result<(), String> {
    let mut regestries: [i32; 4] = [0; 4];
    let stdin = std::io::stdin();
    let instructions = fs::read(filepath).map_err(|err| err.to_string())?;
    let mut pc = 0;
    while pc != instructions.len() {
        if let Some(current_inst) = instructions.get(pc) {
            //println!("{}    {:#010b}", pc, current_inst);
            let op_code = get_op(current_inst);
            match op_code {
                0 => {
                    let (reg1, reg2, imm) = get_r(current_inst);
                    regestries[reg1] += (regestries[reg2] + imm as i32);
                }
                1 => {
                    let (reg1, reg2, imm) = get_r(current_inst);
                    regestries[reg1] -= (regestries[reg2] + imm as i32);
                }
                2 => {
                    let (reg1, reg2, imm) = get_r(current_inst);
                    if regestries[reg1] != regestries[reg2] {
                        pc += 1
                    }
                }
                3 => {
                    let (reg1, imm) = get_i(current_inst);
                    regestries[reg1] += imm as i32
                }
                4 => {
                    let (reg1, imm) = get_i(current_inst);
                    regestries[reg1] -= imm as i32
                }
                5 => {
                    let (reg1, imm) = get_i(current_inst);
                    regestries[reg1] = if imm > 3 {
                        (imm - (7 - imm)).into()
                    } else {
                        imm.into()
                    }
                }
                6 => {
                    let mut sign = current_inst & 0b00010000;
                    sign >>= 4;
                    let amount = (current_inst & 0b00001111) as usize;
                    if sign == 1 {
                        pc -= amount
                    } else {
                        pc += amount
                    };
                    pc -= 1;
                }
                7 => {
                    let call_code = current_inst & 0b00011111;
                    match call_code {
                        0 => println!("{}", regestries[1]),
                        1 => {
                            let input: i32 = stdin
                                .lock()
                                .lines()
                                .next()
                                .ok_or("Error finding input".to_string())?
                                .map_err(|err| err.to_string())?
                                .parse()
                                .map_err(|er: ParseIntError| er.to_string())?;
                            println!("Input recieved: {}", input);
                            regestries[1] = input;
                        }
                        2 => return Ok(()),
                        _ => return Err(format!("Error executing syscall at {}", pc)),
                    }
                }
                _ => {
                    return Err(format!("Error executing instruction at {}", pc));
                }
            }
            pc += 1;
        }
    }

    Err("Error executing executable".to_string())
}

fn get_op(instruction: &u8) -> u8 {
    let mut op_code = instruction & 0b11100000;
    op_code >>= 5;
    op_code
}

fn get_r(instruction: &u8) -> (usize, usize, u8) {
    let mut reg1 = (instruction & 0b00011000) as usize;
    reg1 >>= 3;
    let mut reg2 = (instruction & 0b00000110) as usize;
    reg2 >>= 1;
    let mut imm = instruction & 0b00000001 as u8;
    (reg1, reg2, imm)
}

fn get_i(instruction: &u8) -> (usize, u8) {
    let mut reg1 = (instruction & 0b00011000) as usize;
    reg1 >>= 3;
    let mut imm = instruction & 0b00000111 as u8;
    (reg1, imm)
}
