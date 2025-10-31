#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{ env, fs};
use std::io;

// Instructions
const ADD: u8 = 0b000;
const SUB: u8 = 0b001;
const IFEQ: u8 = 0b010;

const ADDI: u8 = 0b011;
const SUBI: u8 = 0b100;
const LI: u8 = 0b101;

const JUMP: u8 = 0b110;
const SYSCALL: u8 = 0b111;

// Registers
const ZERO: u8 = 0b00;
const IO: u8 = 0b01;
const APPLE: u8 = 0b10;
const ORANGE: u8 = 0b11; 

// Syscall codes
const READ: u8 = 0b00;
const PRINT: u8 = 0b01;
const EXIT: u8 = 0b10;

fn main() {
     

    let args: Vec<String> = env::args().collect();
    let lines: Vec<u8> = fs::read(&args[1]).expect("can't read file");
    //println!("{:?}", lines);

    // registers
    let mut reg: [i32; 4] = [0, 0, 0, 0];

    let mut pc: usize = 0;
    
    // parse and excecute
    while pc < lines.len() {

        // get the operation code and bitshift
        let mut operation = lines[pc] & 0b11100000;
        operation >>= 5;

        if operation == ADD {

            let (rt, rs, imm) = get_rt_rs_imm(lines[pc]);
            reg[rt] = reg[rt] + reg[rs] + imm as i32;

        } else if operation == SUB {

            let (rt, rs, imm) = get_rt_rs_imm(lines[pc]);
            reg[rt] = reg[rt] - reg[rs] - imm as i32;

            
        } else if operation == IFEQ {
            
            let (rt, rs, imm) = get_rt_rs_imm(lines[pc]);
            if reg[rt] != reg[rs] {
                pc +=1
            }

        } else if operation == ADDI {

            let (rt, imm) = get_rt_imm(lines[pc]);
            reg[rt] += imm;

        } else if operation == SUBI {

            let (rt, imm) = get_rt_imm(lines[pc]);
            reg[rt] -= imm;

        } else if operation == LI {
            let (rt, mut imm) = get_rt_imm(lines[pc]);
            
            if imm >= 4 {
                imm -= 4;
                imm *= -1;
            }

            reg[rt] = imm;

        } else if operation == JUMP {

            let mut imm = get_imm(lines[pc]);

            if imm >= 16 {
                imm -= 16;
                pc -= imm;
            } else {
                pc += imm;
            }
            continue;

        } else if operation == SYSCALL {

            let code = get_imm(lines[pc]) as u8;

            match code {
                READ => {
                    let mut buf: String = String::new();
                    io::stdin().read_line(&mut buf).expect("invalid input");
                    reg[IO as usize] = buf.trim().parse().unwrap();},

                PRINT => println!("{:?}", reg[IO as usize]),

                EXIT => break,
                
                _=> println!("invalid syscall code")
            }
        } else {
            println!("invalid op code");
        }
       // println!("Registers: zero: {:?} IO: {:?} A: {:?} O: {:?}", reg[0], reg[1], reg[2], reg[3]);
        pc += 1;
    }



}

/// gets the register codes and imm value
fn get_rt_rs_imm(line: u8) -> (usize, usize, i32){
    let mut rt = (line & 0b00011000) as usize;
    rt >>= 3;
    let mut rs = (line & 0b00000110) as usize;
    rs >>= 1;
    let imm: i32 = (line & 0b00000001) as i32;
    //println!("rt: {:?} rs: {:?} imm: {:?}",rt, rs, imm );
    (rt, rs, imm)
}

/// gets the register code and imm value
fn get_rt_imm(line: u8) -> (usize, i32) {
    let mut rt = (line & 0b00011000) as usize;
    rt >>= 3;
    let imm: i32 = (line & 0b00000111) as i32;

    (rt, imm)
}

fn get_imm(line: u8) -> usize {
    (line & 0b00011111) as usize
}

