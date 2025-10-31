/***
 * Compiler for The Socially Acceptable Language
 * - file and error handling
 */

mod compiler;
mod tests;

use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;

const OUT_FILE_PATH: &str = "./output.salexe";

pub fn compile(arguments: Vec<String>) {
    match fs::read_to_string(arguments[0].clone()) {
        Ok(_contents) => {
            println!("Compiling file: {:?}", arguments[0]);

            let lines: Vec<String> = _contents
                .split("\r\n")
                .map(|_line| _line.to_string())
                .collect();

            
            let mut current_index: usize = 0;
            let mut labels: HashMap<String, usize> = HashMap::new();
            while current_index < lines.len() {
                if line_has_code(&lines[current_index]) {
                    match compiler::find_labels(&lines[current_index]) {
                        Some(_index) => labels.insert(lines[current_index][.._index].to_string(), current_index),
                        None => None
                    };
                }
                current_index += 1;
            }

            current_index = 0;
            let mut executable: Vec<u8> = Vec::new();
            while current_index < lines.len() {
                if line_has_code(&lines[current_index]) {
                    match compiler::run(lines[current_index].as_str(), &labels, &current_index) {
                        Ok(_instruction) => {
                            println!("{:?}:{:08b}", current_index+1, _instruction);
                            executable.push(_instruction)},
                        Err(_) => {
                            println!("{:?}:-", current_index+1);
                        },
                    }
                }
                current_index += 1;
            }

            println!("Done compiling!\nWriting to output...");
            match fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(OUT_FILE_PATH)
            {
                Ok(mut _file) => match _file.write_all(&executable) {
                    Ok(()) => {
                        println!(
                            "Done writing to output\nExecutable code found at: \"{}\"",
                            OUT_FILE_PATH
                        );
                    }
                    Err(_) => {
                        println!("Failed to write file");
                    }
                },
                Err(_) => {
                    println!("Failed to open or create file");
                }
            }
        }
        Err(_) => println!("Failed to read file"),
    }
}

fn line_has_code(line: &str) -> bool {
    !(line.is_empty() || line == line.to_lowercase())
}
