use std::fs;
extern crate bit_vec;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::process;

struct Paths {
    filepath: String,
    define_filepath: String,
    output_filepath: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = Paths::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    run(paths);
}

    

fn run(paths:Paths){
    let file = fs::read_to_string(paths.filepath).expect("Failed to read input file");
    let file = file
        .split("\n")
        .filter(|l| !(l.is_empty()))
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let define_file =
        fs::read_to_string(paths.define_filepath).expect("Failed to read define file");
        
        

    let definitions = load_definition(&define_file);
    //println!("{:?}",file);

    
    let file = replace_macro(&file, &definitions);

    for line in &file {
        println!("{}",line);
    }
    

    let bit_vector = compile(file);

    let mut file = fs::File::create(paths.output_filepath).unwrap();
    file.write_all(&bit_vector);
}

fn set_bits(bitvec: &mut Vec<u8>, index: usize, bits: Vec<u8>) {
    for (i, b) in bits.into_iter().enumerate() {
        bitvec[7 - index + i] = b;
    }
}

fn compile(file: Vec<String>) -> Vec<u8> {
    let mut bvVector: Vec<u8> = Vec::<u8>::new();
    for line in file {
        let mut bv: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        match split_line[0] {
            "sys" => {
                set_bits(&mut bv, 7, vec![0, 1]);
                match split_line[1] {
                    "STACK" => {
                        set_bits(&mut bv, 5, vec![0, 0]);
                        match split_line[2] {
                            "POP" => set_bits(&mut bv, 3, vec![1]),
                            "PUSH" => set_bits(&mut bv, 3, vec![0]),
                            _ => {
                                println!("STACK can only PUSH or POP");
                                process::exit(1);
                            }
                        }
                        let registry_number = get_registry_number(split_line[3]);
                        match registry_number {
                            Ok(v) => set_bits(&mut bv, 2, v),
                            Err(s) => {
                                println!("{}", s);
                                break;
                            }
                        }
                    }
                    "READ" => {
                        set_bits(&mut bv, 5, vec![0, 1]);
                        let registry_number = get_registry_number(split_line[2]);
                        match registry_number {
                            Ok(v) => set_bits(&mut bv, 3, v),
                            Err(s) => {
                                println!("{}", s);
                                break;
                            }
                        }
                    }
                    "WRITE" => {
                        set_bits(&mut bv, 5, vec![1, 0]);
                        let registry_number = get_registry_number(split_line[2]);
                        match registry_number {
                            Ok(v) => set_bits(&mut bv, 3, v),
                            Err(s) => {
                                println!("{}", s);
                                break;
                            }
                        }
                    }
                    _ => {
                        println!("no such syscall");
                        break;
                    }
                }
            }
            "START" => {
                set_bits(&mut bv, 7, vec![1, 0]);
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number {
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s) => {
                        println!("{}", s);
                        break;
                    }
                }

                let registry2_number = get_registry_number(split_line[2]);
                match registry2_number {
                    Ok(v) => set_bits(&mut bv, 2, v),
                    Err(s) => {
                        println!("{}", s);
                        break;
                    }
                }
            }
            "END" => {
                set_bits(&mut bv, 7, vec![1, 0]);
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number {
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s) => {
                        println!("{}", s);
                        process::exit(1);
                    }
                }
            }
            "NAND" => {
                set_bits(&mut bv, 7, vec![0, 0]);
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number {
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s) => {
                        println!("{}", s);
                        process::exit(1);
                    }
                };
                let registry2_number = get_registry_number(split_line[2]);
                match registry2_number {
                    Ok(v) => set_bits(&mut bv, 2, v),
                    Err(s) => {
                        println!("{}", s);
                        process::exit(1);
                    }
                }
            }
            "BIT" => {
                set_bits(&mut bv, 7, vec![1, 1]);
                match split_line[1] {
                    "READ" => set_bits(&mut bv, 5, vec![0]),
                    "WRITE" => set_bits(&mut bv, 5, vec![1]),
                    _ => {
                        panic!("BIT has no such command {}", split_line[1]);
                    }
                }

                let imm = format!("{:0>5b}", split_line[2].parse::<u8>().unwrap())
                    .chars()
                    .map(|c| c.to_digit(2).unwrap() as u8)
                    .collect::<Vec<u8>>();
                set_bits(&mut bv, 4, imm);
            }
            _ => println!("unknown command {}", split_line[0]),
        }
        let mut number: u8 = 0;
        let base: u8 = 2;
        for (i, b) in bv.iter().enumerate() {
            number += base.pow((7 - i) as u32) * b;
            //print!("{}",b);
        }
        //println!();
        bvVector.push(number);
    }

    return bvVector;
}

fn get_registry_number(registry: &str) -> Result<Vec<u8>, String> {
    let mut index_registry;
    match registry {
        "pc" => index_registry = 0,
        "rn" => index_registry = 1,
        "r0" => index_registry = 2,
        "r1" => index_registry = 3,
        "r2" => index_registry = 4,
        "r3" => index_registry = 5,
        "r4" => index_registry = 6,
        "r5" => index_registry = 7,
        _ => {
            println!("no such registry {}", registry);
            return Err(String::from("no such registry"));
        }
    }
    let mut s = format!("{:0>3b}", index_registry);

    return Ok(s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>());
}

fn replace_macro<'a>(
    nand_file: &'a Vec<String>,
    definitions: &'a HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut replaced_file = Vec::<String>::new();
    //println!("#######################");

    for instruction in nand_file {
        let instruction=clear_comments(instruction);
        let split_instruction: Vec<&str> = instruction.split_whitespace().collect();
        if instruction.is_empty(){continue;}
        if definitions.contains_key(split_instruction[0]) {
            let mut definition = definitions.get(split_instruction[0]).unwrap().clone();
            let number_of_args = definition[0].parse::<usize>().unwrap();
            definition.remove(0);

            if split_instruction.len() - 1 == number_of_args {
                let mut replaced_instructions = definition.clone();
                let args: Vec<&str> = definition[0].split_whitespace().collect();
                replaced_instructions.remove(0);
                

                let mut counter = 0;

                loop {
                    replaced_instructions[counter]=replace(&args, &replaced_instructions[counter], &split_instruction);
                    counter += 1;
                    if counter == replaced_instructions.len(){
                        break;
                    }
                    
                }
                for i in 0..replaced_instructions.len() {
                    replaced_file.push(replaced_instructions[i].clone());
                }
            } else {
                panic!(
                    "Cant call macro {} with {} args. Takes {} args",
                    split_instruction[0],
                    split_instruction.len() - 1,
                    number_of_args
                );
            }
        } else {
            replaced_file.push(instruction.clone());
        }
    }
    return replaced_file;
}

fn load_definition(define_file: &String) -> HashMap<String, Vec<String>> {
    let define_file = define_file
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    let mut in_definition = false;
    let mut definition: Vec<String> = vec![];
    let mut definition_name: String = String::new();
    let mut definitions: HashMap<String, Vec<String>> = HashMap::new();

    for instruction in define_file {
        let instruction=clear_comments(instruction);
        let mut split_instructions: Vec<&str> = instruction.split_whitespace().collect();
        if split_instructions[0] == ".end_define" {
            in_definition = false;
            definitions.insert(definition_name.clone(), definition.clone());
        } else if in_definition == true {
            //definition = format!("{}\n{}", definition, instruction)
            definition.push(instruction.to_string())
        } else if split_instructions[0] == ".define" {
            definition = Vec::new();
            definition_name = String::new();

            in_definition = true;
            definition_name = split_instructions[1].to_string();
            let number_args = split_instructions.len() - 2;

            definition.push(number_args.to_string());
            split_instructions.remove(0);
            split_instructions.remove(0);

            definition.push(split_instructions.join(" ").to_string());
        }
    }

    let mut found_macro = true;
    while found_macro {
        found_macro = false;
        let temp_def = definitions.clone();
        for def in &mut definitions {
            let new_def = replace_macro(&def.1, &temp_def);
            //println!("def : {:?}", &def.1);
            //println!("new_def; {:?}", new_def);
            if &new_def != def.1 {
                *def.1 = new_def;
                found_macro = true;
            }
        }
    }

    return definitions;
}

impl Paths {
    fn new(args: &[String]) -> Result<Paths, String> {
        let mut filepath: String = String::from("AND.asm");
        let mut output_filepath: String = String::from("a.nand");
        let mut define_filepath: String = String::from("define.asm");

        if args.len() > 1 {
            for (i, arg) in args.iter().enumerate() {
                if arg.chars().next().unwrap() == '-' {
                    match arg.chars().nth(1).unwrap() {
                        'o' => {
                            if i >= args.len() - 1 {
                                return Err(String::from("Please specify output name"));
                            }
                            output_filepath = args[i + 1].clone();
                        }
                        _ => return Err(String::from("No such flag")),
                    }
                } else {
                    if arg == &output_filepath {
                        continue;
                    }
                    filepath = arg.clone();
                }
            }
        }

        return Ok(Paths {
            filepath,
            define_filepath,
            output_filepath,
        });
    }
}

fn replace(args: &Vec<&str>, replaced: &String, split_instruction: &Vec<&str>) -> String{
    let mut repl_split: Vec<String> = replaced.split_whitespace().map(|s|s.to_string()).collect();
    for i in 0..repl_split.len() {
        for j in 0..args.len() { 
                let reg_to_replace = args[j];
                let tmp_str= repl_split[i].replace(reg_to_replace, split_instruction[j + 1]).clone();
                if tmp_str != repl_split[i]{
                    repl_split[i]=tmp_str;
                    break;
                }
                      
        }
    }

    return repl_split.join(" ").to_string();
}

fn clear_comments(instruction: &str)-> String{
    return instruction.split("#").next().unwrap().to_string();
}

//for testing
fn pause(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
}
