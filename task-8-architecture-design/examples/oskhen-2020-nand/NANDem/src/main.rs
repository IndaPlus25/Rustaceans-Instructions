#![allow(dead_code)]

use std::{env, fs, io, convert::TryInto};
use u32;

// TODO: Pass references of original binary instead of cloning everywhere

fn main() {

    let binary = load_binary();
    //println!("binary: {:?}", binary);
    emulate_program(binary);
    


}

fn emulate_program(binary: Vec<String>) {
    
    // -| Init processor

    static mut STACK:Vec<u32> = Vec::<u32>::new();

    static mut REGISTERS: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    static mut PROCESS:Vec<String> = Vec::<String>::new();

    static mut PROCESSTACK:Vec<Vec<String>> = Vec::<Vec<String>>::new();

    static mut REGISTRYSTACK:Vec<u32> = Vec::<u32>::new();

    // -|

    fn run_process(instructions: Vec<String>) {
        
        let mut reached_end = false;

        unsafe {
            PROCESSTACK.push(PROCESS.clone());
            PROCESS = instructions.clone();
        }

        unsafe {

            while REGISTERS[0] < PROCESS.len().try_into().expect("File size too large!") && reached_end == false {
                reached_end = parse_instruction(PROCESS[REGISTERS[0] as usize].clone());
            }
            
            //println!("Reached end of process for {:?}", PROCESS);

            if PROCESSTACK.len() != 0 {
                PROCESS = PROCESSTACK.pop().unwrap();
                //println!("Returning to process {:?}", PROCESS);
            }

        }
        
    }
    
    fn parse_instruction(instruction: String) -> bool {
        
        let mut reached_end = false;
        unsafe {
            //println!();
            //println!("REGISTERS: {:?}", REGISTERS);
            //println!("STACK: {:?}", STACK);
            //println!("PROCESS: {:?}", PROCESS);
            //println!("PROCESSSTACK: {:?}", PROCESSTACK);
            //println!("REGISTRYSTACK: {:?}", REGISTRYSTACK);
            //println!("Current instruction: {}", instruction);
        }

        let op = &instruction[0..2];
        match op {
            "00" => parse_nand(instruction),
            "01" => parse_sys(instruction),
            "10" => {
                let id = &instruction[5..8];
                match id {
                    "000" => {
                        parse_end(instruction);
                        reached_end = true;
                    }
                    _ => {
                        let processclone = unsafe {PROCESS.clone()};
                        parse_start(instruction, processclone);
                    }
                }

            },
            "11" => {
                parse_bit(instruction);
            },
            _ => panic!("Invalid OP code {}", op),
        };
        reached_end
    }

    fn parse_bit(instruction: String) {

        let io = &instruction[2..3];
        let imm = usize::from_str_radix(&instruction[3..8], 2).unwrap();

        // Read
        if io == "0" {
            unsafe {
                REGISTERS[7] = format!("{:0>32b}", REGISTERS[6]).chars().nth(31-imm).unwrap().to_digit(2).unwrap();
            }
            
        }
        // Write
        else {
            unsafe {
                let mut binr4: Vec<char> = format!("{:0>32b}", REGISTERS[6]).chars().collect();
                let r5bit = format!("{:0>32b}", REGISTERS[7]).chars().last().unwrap();
                binr4[31-imm] = r5bit; // Big endian
                let u32r4: String = binr4.into_iter().collect();
                REGISTERS[6] = u32::from_str_radix(&u32r4, 2).unwrap();
            }
        }

        unsafe { REGISTERS[0] += 1; }

    }

    fn parse_nand(instruction: String) {

        let rt = usize::from_str_radix(&instruction[2..5], 2).unwrap();
        let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();

        unsafe { 
            REGISTERS[1] = !(REGISTERS[rt]&REGISTERS[rs]);
            REGISTERS[0] += 1;
        }
        

    }

    fn parse_sys(instruction: String) {

        let id = &instruction[2..4];
        match id {
            "00" => {
                let STACKtype = &instruction[4..5];
                let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();
                if STACKtype == "0" {
                    unsafe { STACK.push(REGISTERS[rs]) }
                }
                else {
                    unsafe { REGISTERS[rs] = STACK.pop().expect("Tried popping from an empty STACK!") }
                }
            },
            "01" => {
                let rs = usize::from_str_radix(&instruction[4..7], 2).unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input!");
                let input = input.trim().parse::<u32>().expect("Expected u32 integer!");
                unsafe { REGISTERS[rs] = input }
            }
            "10" => {
                let rs = usize::from_str_radix(&instruction[4..7], 2).unwrap();
                unsafe { 
                    println!("{}", REGISTERS[rs] ) 
                }
            }
            _ => panic!("Invalid instruction at {}!", instruction),
        };

        unsafe { REGISTERS[0] += 1; }
    }

    fn parse_start(instruction: String, process: Vec<String>) {

        let rt = usize::from_str_radix(&instruction[2..5], 2).unwrap();
        let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();

        let mut subprocess = Vec::<String>::new();

        unsafe {
            let mut depthcounter = 1;
            for i in ((REGISTERS[0]+1) as usize)..process.len() {
                let op = &process[i][0..2];
                subprocess.push(process[i].clone());
                if op == "10" {
                    if &process[i][5..8] == "000" {
                        depthcounter -= 1;
                        if depthcounter == 0 {
                            break;
                        }
                        
                    }
                    else {
                        depthcounter += 1;
                    }
                    
                }
                
            }   

            let savedrt = REGISTERS[rt];
            let savedrs = REGISTERS[rs];

            for i in 0..8 {
                REGISTRYSTACK.push(REGISTERS[i]);
                REGISTERS[i] = 0;
            }


            REGISTERS[2] = savedrt;
            REGISTERS[3] = savedrs;

        }

        run_process(subprocess);

    }

    fn parse_end(instruction: String) {

        let rt = usize::from_str_radix(&instruction[2..5], 2).unwrap();
        
        unsafe {
            let savedrt = REGISTERS[1];
            
            for i in (1..8).rev() {
                REGISTERS[i] = REGISTRYSTACK.pop().unwrap();
            }

            let orig: u32 = REGISTRYSTACK.pop().unwrap();
            let offset: u32 = PROCESS.len().try_into().unwrap();
            REGISTERS[0] = orig + offset + 1;
            
            REGISTERS[rt] = savedrt;
        }

    }


    run_process(binary);

    println!("Reached EOF!");

} 

fn load_binary() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Requires file to run!")
    }
    let filepath = &args[1];
    let file = fs::read(filepath).expect("Failed to read file");
    let mut instructionvec = Vec::<String>::new();

    for rawinstruction in file.iter() {
        let instruction = format!("{:0>8b}", rawinstruction);
        instructionvec.push(instruction);
    }

    instructionvec

}