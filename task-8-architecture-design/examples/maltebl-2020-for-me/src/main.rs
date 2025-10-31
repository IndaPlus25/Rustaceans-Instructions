use std::{env, fs};
mod compiler;
use compiler::compile;
mod emulator;
use emulator::execute;
mod language_specs;
fn main() {
    let mut args = env::args();
    args.next();
    if let (Some(arg), Some(filepath)) = (args.next(), args.next()) {
        if arg == "compile" {
            if let (Ok(code), Some(output_file)) =
                (fs::read_to_string(filepath + ".forme"), args.next())
            {
                compile(code, &output_file).unwrap();
            } else {
                println!("Error reading code file and or output file path...");
                panic!();
            }
        } else if arg == "run" {
            execute(filepath + ".formexe").unwrap();
        }
    } else {
        println!("Error parsing arguments...")
    }
}
