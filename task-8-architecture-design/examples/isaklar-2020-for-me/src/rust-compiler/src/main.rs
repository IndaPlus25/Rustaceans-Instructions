
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// packages
use std::{ env, fs};
use std::collections::HashMap;
use std::io::prelude::*;

// Instructions
const ADD: Operation = (OpType::R, 0,0,0);
const SUB: Operation = (OpType::R, 0,0,1);
const IFEQ: Operation = (OpType::R, 0,1,0);

const ADDI: Operation = (OpType::I, 0,1,1);
const SUBI: Operation = (OpType::I, 1,0,0);
const LI: Operation = (OpType::I, 1,0,1);

const JUMP: Operation = (OpType::J, 1,1,0);
const SYSCALL: Operation = (OpType::J, 1,1,1);

// Registers
const ZERO: (u8, u8) = (0,0);
const IO: (u8, u8) = (0,1);
const APPLE: (u8, u8) = (1,0);
const ORANGE: (u8, u8) = (1,1); 

// Syscall codes
const READ: u8 = 0;
const PRINT: u8 = 1;
const EXIT: u8 = 2;

#[derive(PartialEq)]
enum OpType {
    R,
    I,
    J
}

type Operation = (OpType, u8, u8, u8);

fn main() {

    // get input file
    let args: Vec<String> = env::args().collect();
    let lines = fs::read_to_string(&args[1]).expect("Failed to read file contents");

    // clean input 
    let mut lines: Vec<&str> = lines.split("\r\n").collect();
    lines.retain(|&s| s != "" && !s.starts_with("#"));

    // first pass to define the labels
    let mut labels: HashMap<&str, usize> = HashMap::new();

    //println!("");
    let mut i: usize = 0;
    while i < lines.len() {
        if lines[i].ends_with(":") {
            labels.insert(lines[i].trim_end_matches(":"), i);
            lines.remove(i);
            
            continue;
        }
        i += 1;
    }
    //println!("The labels are: {:?}", labels);
    let mut output: Vec<u8> = Vec::new();

    // second pass 
    for i in 0..lines.len() {

        let components: Vec<&str> = lines[i].split_whitespace().collect();
        let op = instruction_lookup(components[0]);

        let mut instruction: u8 = 0;

        if op.0 == OpType::R {

            let rt = register_lookup(components[1]);
            let rs = register_lookup(components[2]);
            let imm: u8 = components[3].parse().unwrap();

            // add the converted binary code
            instruction += op.1*2u8.pow(7) + op.2*2u8.pow(6) + op.3*2u8.pow(5) + rt.0*2u8.pow(4) + rt.1*2u8.pow(3) + rs.0*2u8.pow(2) + rs.1*2u8.pow(1) + imm*2u8.pow(0);


        } else if op.0 == OpType::I {

            let rt = register_lookup(components[1]);
            let imm: i8 = components[2].parse().unwrap();
            let sign = if imm < 0 {1} else {0};

            // add the converted binary code with sign
            instruction += op.1 * 2u8.pow(7) + op.2*2u8.pow(6) + op.3*2u8.pow(5) + rt.0*2u8.pow(4) + rt.1*2u8.pow(3) + sign*2u8.pow(2) + imm.abs() as u8;
            
            
            
        } else if op.0 == OpType::J {
            if op == SYSCALL {
                // if SYSCALL
                let call_code: u8 = syscall_lookup(components[1]);

                instruction += op.1 * 2u8.pow(7) + op.2*2u8.pow(6) + op.3*2u8.pow(5) + call_code;

            }else {
                // jump instruction

                let from: i8 = i as i8;
                let to: usize = 0 + labels.get(components[1]).unwrap();

                let jump: i8 = to as i8 - from;
                let sign = if jump < 0 {1} else {0};

                //println!("op: {:?} {:?} {:?}, jump: {:?} sign: {:?}",op.1, op.2, op.3, (jump ), sign);

                instruction += (op.1 * (2u8.pow(7))) + op.2*2u8.pow(6) + op.3*2u8.pow(5) + sign*2u8.pow(4) + (jump.abs() as u8);
                
            }
        }

        output.push(instruction);
    }
/*    for n in output {
        println!("{:#010b}  {:?}", n, n);

    }
*/
    // create excecutable
    let mut file = fs::OpenOptions::new().write(true).create(true).open("../program.formexe").expect("creating file failed");
    file.write_all(&output).expect("writing to file failed");
}

/// gets the binary code and OpType of the instruction
fn instruction_lookup(symbol: &str) -> Operation {
    match symbol {
        "💘" => ADD,
        "💔" => SUB,
        "🤔" => IFEQ,
        "👆" => ADDI,
        "👇" => SUBI,
        "👈" => LI,
        "♿" => JUMP,
        "🤖" => SYSCALL,
        &_ => {println!("unknown instruction"); ADD}
    }
}

/// gets the binary code of the register
fn register_lookup(symbol: &str) -> (u8, u8) {
    match symbol {
        "🍩" => ZERO,
        "👀" => IO,
        "🍎" => APPLE,
        "🍊" => ORANGE,
        &_ => {println!("invalid register"); ZERO}
    }
}

fn syscall_lookup(symbol: &str) -> u8 {
    match symbol {
        "📢" => PRINT,
        "📜" => READ,
        "🔪" => EXIT, 
        &_ => {println!("invalid call code"); EXIT} 
    }
}
