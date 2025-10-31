// Shut up rust
#![allow(dead_code)] 
#![allow(unused)]
#![allow(warnings)]

use std::io;
use std::io::prelude::*;
use std::env;
use std::fs;

// Registers
static mut register: [i32; 4] = [0; 4];
static mut pc: i32 = 0;

// Holds all commands incase of jumps
static mut command: Vec<String> = Vec::new();

// Instructions
const ADD: (bool, bool, bool, bool) = (true, false, false, false); // (1, 0, 0, 0) - 2 following registers
const SUB: (bool, bool, bool, bool) = (true, true, false, false); // (1, 1, 0, 0) - 2 following registers
const MOVE: (bool, bool, bool, bool) = (true, false, true, false); // (1, 0, 1, 0), - 2 following registers

const LI: (bool, bool, bool) = (false, false, true); // (0, 0, 1), - following 5 bit immediate
const JR: (bool, bool, bool) = (true, true, true); // (1, 1, 1), - following 5 bit immediate
const BNE: (bool, bool, bool) = (false, true, true); // (0, 1, 1) - following 5 bit immediate // Always compare register 3 and 4 - should probs be changed to reg2 < reg3

const READ: (bool, bool, bool, bool, bool, bool) = (false, false, false, false, true, false); // (0, 0, 0, 0, 1, 0) - following register
const WRITE: (bool, bool, bool, bool, bool, bool) = (false, false, false, false, false, false); // (0, 0, 0, 0, 0, 0) - following register
const ADD1: (bool, bool, bool, bool, bool, bool) = (false, false, false, false, false, true); // (0, 0, 0, 0, 0, 1) - following register // PRO GAMER MOVE
const SKIP: (bool, bool, bool, bool, bool, bool, bool, bool) = (false, false, false, false, true, true, false, false); // (0, 0, 0, 0, 1, 1, 0, 0) - following register // PRO GAMER MOVE

// Interpreter for barcode
fn main() {

    // Fills command with commands
    read_input();

    // You can seperate this interpreter into a compiler and an emulator:
    // Compiler-ish: read_input fills "command: Vec<String>", just print all strings into a txt file and you have binary code.
    // Emulator-ish: Fill "command: Vec<String>" by reading the txt file and splitting every 8th bit as a string and run the loop below.

    unsafe{
        // Read commands untill there are no more, there is no exit command...
        while(pc < command.len() as i32) {
            read_command(pc as usize);
            pc += 1;
        }
    };
}

// Read char
fn read_char(c: char) -> bool {
    match c {
        '0' => return false,
        '1' => return true,
        _ => return false // HOW DID YOU END UP HERE?
    }
}

// tuple of booleans to number - dumbass that I am didnt leave my tuples as 0 and 1 so that I could sum = 2^0 + 2^1...
fn read_register(a: bool, b: bool) -> usize {
    let mut sum = 0;
    if a {sum += 2}
    if b {sum += 1}
    return sum;
}

// tuple of booleans to number - dumbass that I am didnt leave my tuples as 0 and 1 so that I could sum = 2^0 + 2^1... // I have gone past the point of no return
fn read_immediate(a: bool, b: bool, c: bool, d: bool, e: bool) -> i32 {
    let mut sum = 0;
    if b {sum += 8}
    if c {sum += 4}
    if d {sum += 2}
    if e {sum += 1}
    if a {sum *= -1}
    return sum;
}

// Read input
fn read_input() {

    // Get standard input stream
    let input = io::stdin();

    // Get input file - thank you Isak
    let args: Vec<String> = env::args().collect();
    let lines = fs::read_to_string(&args[1]).expect("Failed to read file contents");

    // Split code and remove all comments from code
    let mut code: Vec <&str> = lines.split("ii").collect();
    for x in (0..code.len()).filter(|x| x % 2 == 1) {
        code[x] = "";
    }

    // Connect all lines
    let mut file: String = String::new();
    for string in code {
        file += string;
    }

    // Split them into 8 char long commands
    let mut i: usize = 0;
    let mut n: usize = 0;
    for c in file.chars() {

        // Add character to command string
        unsafe { // idc rust, leave me alone
            match c {
                'l' => {
                    // Modulo to ensure all commands are 8 bits
                    if i % 8 == 0 && i / 8 == command.len() {
                        n += 1;
                        command.push(String::new());
                    }
                    command[n - 1].push('1');
                },
                'I' => {
                    // Modulo to ensure all commands are 8 bits
                    if i % 8 == 0 && i / 8 == command.len() {
                        n += 1;
                        command.push(String::new());
                    }
                    command[n - 1].push('0');
                },
                _ => continue // Incase some dumbass doesn't mark their comments with ii "..." ii
            }
        };

        // Length of command
        i += 1;
    }
}

fn read_command(n: usize) {
    unsafe { // Too bad

        // Copy command to read every bit
        let mut commandcpy = command[n].chars();
    

        // Instruction
        let mut is: (bool, bool, bool, bool, bool, bool, bool, bool) = (false, false, false, false, false, false, false, false); // Cursed

        // Read seperate bits
        for n in 0..8 {
            match n {
                0 => is.0 = read_char(commandcpy.next().unwrap()),
                1 => is.1 = read_char(commandcpy.next().unwrap()),
                2 => is.2 = read_char(commandcpy.next().unwrap()),
                3 => is.3 = read_char(commandcpy.next().unwrap()),
                4 => is.4 = read_char(commandcpy.next().unwrap()),
                5 => is.5 = read_char(commandcpy.next().unwrap()),
                6 => is.6 = read_char(commandcpy.next().unwrap()),
                7 => is.7 = read_char(commandcpy.next().unwrap()),
                _ => {} // Unreachable
            }
        }

        // Instruction matching
        if is.0 == ADD.0 && is.1 == ADD.1 && is.2 == ADD.2 && is.3 == ADD.3 {
            // Registers
            let rd = read_register(is.4, is.5);
            let rt = read_register(is.6, is.7);

            add(rd, rt);

        } else if is.0 == SUB.0 && is.1 == SUB.1 && is.2 == SUB.2 && is.3 == SUB.3 {
            // Registers
            let rd = read_register(is.4, is.5);
            let rt = read_register(is.6, is.7);

            sub(rd, rt);

        } else if is.0 == MOVE.0 && is.1 == MOVE.1 && is.2 == MOVE.2 && is.3 == MOVE.3 {
            // Registers
            let rd = read_register(is.4, is.5);
            let rt = read_register(is.6, is.7);

            mv(rd, rt);

        } else if is.0 == LI.0 && is.1 == LI.1 && is.2 == LI.2 {
            //Immediate
            let n = read_immediate(is.3, is.4, is.5, is.6, is.7);

            li(n);

        } else if is.0 == JR.0 && is.1 == JR.1 && is.2 == JR.2 {
            //Immediate
            let n = read_immediate(is.3, is.4, is.5, is.6, is.7);

            jr(n);

        } else if is.0 == BNE.0 && is.1 == BNE.1 && is.2 == BNE.2 {
            //Immediate
            let n = read_immediate(is.3, is.4, is.5, is.6, is.7);

            bne(n);

        } else if is.0 == READ.0 && is.1 == READ.1 && is.2 == READ.2 && is.3 == READ.3 && is.4 == READ.4 && is.5 == READ.5 {
            //Register
            let rd = read_register(is.6, is.7);

            read(rd);

        } else if is.0 == WRITE.0 && is.1 == WRITE.1 && is.2 == WRITE.2 && is.3 == WRITE.3 && is.4 == WRITE.4 && is.5 == WRITE.5 {
            // Registers
            let rd = read_register(is.6, is.7);

            write(rd);

        } else if is.0 == ADD1.0 && is.1 == ADD1.1 && is.2 == ADD1.2 && is.3 == ADD1.3 && is.4 == ADD1.4 && is.5 == ADD1.5 {
            // Registers
            let rd = read_register(is.6, is.7);

            add1(rd);

        } else if is.0 == SKIP.0 && is.1 == SKIP.1 && is.2 == SKIP.2 && is.3 == SKIP.3 && is.4 == SKIP.4 && is.5 == SKIP.5 && is.6 == SKIP.6 && is.7 == SKIP.7 {
            skip();
        }
    };
}

// Instructions as functions
fn add(rd: usize, rt: usize) {
    unsafe {
        register[rd] += register[rt];
    };
}

fn sub(rd: usize, rt: usize) {
    unsafe {
        register[rd] -= register[rt];
    };
}

fn mv(rd: usize, rt: usize) {
    unsafe {
        register[rt] = register[rd];
    };
}

fn li(imm: i32) {
    unsafe {
        register[0] = imm;
    };
}

fn jr(imm: i32) {
    unsafe {
        pc += imm;
    };
}

fn bne(imm: i32) {
    unsafe {
        if register[2] != register[3] {
            jr(imm);
        }
    };
}

fn read(rd: usize) {
    print!("\nEnter an i32:\n");
    let mut not_found: bool = true;

    // Tries to read input
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    while(not_found) {
        

        // Cheching that input is an i32
        if s == "" {
            continue;
        } else {
            match s.trim().parse::<i32>() {
                Ok(value) => {
                    unsafe {
                        register[rd] = value;
                    };
                    not_found = false;
                },
                Err(err) => {
                    print!("\n{:?}\n", err);
                    s.clear();
                }
            }
        }
    };
}

fn write(rd: usize) {
    unsafe {
        print!("\n{:?}\n", register[rd]);
    };
}

fn add1(rd: usize) {
    unsafe {
        register[rd] += 1;
    };
}

fn skip() {
    unsafe {
        if register[2] == register[3] {
            jr(1);
        }
    };
}