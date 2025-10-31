use std::fs;
extern crate bit_vec;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;

fn main() {
    let filepath: &str;
    let define_filepath: &str;
    let output_path: &str;

    let args: Vec<String> = env::args().collect();

    let arg_parsing = parse_args(&args);
    if arg_parsing.is_ok() {
        let tuple = arg_parsing.unwrap();
        filepath = tuple.0;
        output_path = tuple.1;
        define_filepath = tuple.2;
    } else {
        println!("{}", arg_parsing.err().unwrap());
        return;
    }

    let file = fs::read_to_string(filepath).expect("Failed to read input file");
    let file = file
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    let define_file = fs::read_to_string(define_filepath).expect("Failed to read define file");
    let define_file = define_file
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    let definitions = load_definition(&define_file);
    let file = replace_macro(&file, &definitions);
    let bit_vector = compile(file);
    let mut file = fs::File::create(output_path).unwrap();
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
                                break;
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
                set_bits(&mut bv, 7, vec![1, 1]);
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number {
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s) => {
                        println!("{}", s);
                        break;
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
                        break;
                    }
                };
                let registry2_number = get_registry_number(split_line[2]);
                match registry2_number {
                    Ok(v) => set_bits(&mut bv, 2, v),
                    Err(s) => {
                        println!("{}", s);
                        break;
                    }
                }
            }
            _ => println!("unknown command {}", split_line[0]),
        }
        let mut number: u8 = 0;
        let base: u8 = 2;
        for (i, b) in bv.iter().enumerate() {
            number += base.pow((7 - i) as u32) * b
        }
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
            println!("no such registry");
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
    nand_file: &'a Vec<&str>,
    definitions: &'a HashMap<String, String>,
) -> Vec<String> {
    let mut replaced_file = Vec::<String>::new();

    for line in nand_file {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        if definitions.contains_key(split_line[0]) {
            let definition = definitions.get(split_line[0]).unwrap().clone();
            let number_of_args = definition.chars().next().unwrap().to_digit(10).unwrap() as usize;

            if split_line.len() - 1 == number_of_args {
                let mut replaced_lines = definition.clone();
                let args: Vec<&str> = definition.split_whitespace().collect();
                for i in 1..split_line.len() {
                    let reg_to_replace = args[i];
                    replaced_lines = replaced_lines.replace(reg_to_replace, split_line[i]);
                }
                let replaced_lines: Vec<String> =
                    replaced_lines.split("\n").map(|l| l.to_string()).collect();

                for i in 1..replaced_lines.len() {
                    replaced_file.push(replaced_lines[i].clone());
                }
            }
        } else {
            replaced_file.push(line.to_string());
        }
    }
    println!("{:?}", replaced_file);
    return replaced_file;
}

fn load_definition(define_file: &Vec<&str>) -> HashMap<String, String> {
    let mut in_definition = false;
    let mut definition = String::new();
    let mut definition_name: String = String::new();
    let mut definitions: HashMap<String, String> = HashMap::new();

    for line in define_file {
        let split_lines: Vec<&str> = line.split_whitespace().collect();
        if split_lines[0] == ".end_define" {
            in_definition = false;
            definitions.insert(definition_name.clone(), definition.clone());
            definition = String::new();
            definition_name = String::new();
        } else if in_definition == true {
            definition = format!("{}\n{}", definition, line)
        } else if split_lines[0] == ".define" {
            in_definition = true;
            definition_name = split_lines[1].to_string();
            let number_args = split_lines.len() - 2;

            definition = format!(
                "{} {}",
                number_args,
                split_lines[2..split_lines.len()].join(" ").trim()
            );
        }
    }
    return definitions;
}

fn parse_args(args: &[String]) -> Result<(&str, &str, &str), String> {
    let mut input_path: &str = "./AND.asm";
    let mut output_path: &str = "./a.nand";
    let mut define_path: &str = "./define.asm";

    if args.len() > 1 {
        for (i, arg) in args.iter().enumerate() {
            if arg.chars().next().unwrap() == '-' {
                match arg.chars().nth(1).unwrap() {
                    'o' => {
                        if i >= args.len() - 1 {
                            return Err(String::from("Please specify output name"));
                        }
                        output_path = &args[i + 1];
                    }
                    _ => return Err(String::from("No such flag")),
                }
            } else {
                if arg==output_path{
                    continue;
                }
                input_path = &arg;
            }
        }
    }

    return Ok((input_path, output_path, define_path));
}
