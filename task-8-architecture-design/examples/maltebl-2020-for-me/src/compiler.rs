use crate::language_specs::*;
use std::{fs, io::Write};

pub fn compile(code: String, output_name: &str) -> Result<(), String> {
    let macros = parse_macros(&code)?;
    let code = insert_macros(code, macros)?;
    let mut labels: Vec<(String, u32)> = Vec::new();
    find_labels(&code, &mut labels);
    let code_lines = code.lines().map(|l| {
        l.split_whitespace()
            .take_while(|w| !w.starts_with('#'))
            .map(|w| w.to_string())
    });
    let mut executable: Vec<u8> = Vec::new();
    let mut pc = 0;
    for line in code_lines {
        if let Some(bits) = parse_line(line, &mut labels, &mut pc)
            .map_err(|err| err + format!(" at line {}", pc).as_str())?
        {
            executable.push(bits);
        }
    }
    make_executable(executable, &output_name)
}

fn make_executable(binary: Vec<u8>, output_name: &str) -> Result<(), String> {
    if let Ok(mut file) = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_name.to_string() + ".formexe")
    {
        return file
            .write_all(binary.as_slice())
            .map_err(|err| err.to_string());
    }
    Err("Error making executable".to_string())
}

fn parse_macros(code: &str) -> Result<Vec<(String, String)>, String> {
    let code = code.lines();
    let mut in_macro = false;
    let mut macro_name = String::new();
    let mut macr: String = String::new();
    let mut macros: Vec<(String, String)> = Vec::new();
    for line in code {
        if in_macro {
            if line.contains(END_MACRO) {
                macros.push((macro_name.clone(), macr));
                macro_name = String::new();
                macr = String::new();
                in_macro = false;
            } else {
                macr += "\n";
                macr += line;
            }
        } else if line.contains(MACRO) {
            let mut line = line.split_whitespace();
            line.next();
            macro_name = line
                .next()
                .ok_or("Error parsing macro name".to_string())?
                .to_string();
            in_macro = true;
        } else if line.contains(CODE) {
            return Ok(macros);
        }
    }
    Err("Error parsing macros".to_string())
}

fn insert_macros(code: String, macros: Vec<(String, String)>) -> Result<String, String> {
    let mut cod = String::new();
    let mut code = code
        .lines()
        .skip_while(|w| !w.contains(CODE))
        .map(|l| l.to_string());
    code.next();
    for mut line in &mut code {
        for (name, macr) in &macros {
            if line.contains(name.as_str()) {
                let mut words = line.split_whitespace();
                words.next();
                let input = words
                    .next()
                    .ok_or("error parsing macro input".to_string())?;
                let mut maca = macr.replace("_", input);
                if maca.contains("¤") {
                    let input2 = words
                        .next()
                        .ok_or("error parsing macro input".to_string())?;
                    maca = maca.replace("¤", input2);
                }
                line = maca;
            }
        }
        cod += (line + "\n").as_str();
    }
    Ok(cod)
}

fn find_labels(code: &str, labels: &mut Vec<(String, u32)>) {
    let mut code = code.lines().map(|l| l.to_string());
    let mut pc = 0;
    for line in &mut code {
        if let Some(word) = line.split_whitespace().next() {
            if word.ends_with(':') {
                let word = word.replace(":", "");
                labels.push((word, pc));
            } else {
                pc += 1;
            }
        }
    }
}

fn parse_line<I: Iterator<Item = String>>(
    mut line: I,
    labels: &mut Vec<(String, u32)>,
    pc: &mut u32,
) -> Result<Option<u8>, String> {
    let mut instruction: u8 = 0;
    if let Some(word) = line.next() {
        if word.ends_with(':') {
            return Ok(None);
        } else {
            let op_code = parse_op(word)?;
            instruction += op_code;
            if op_code <= 64 {
                instruction += parse_r(line)?;
            } else if op_code <= 160 {
                instruction += parse_i(line)?;
            } else if op_code == 192 {
                instruction += parse_jump(line, labels, *pc)?;
            } else {
                instruction += parse_call(line)?;
            }
            *pc += 1;
            return Ok(Some(instruction));
        }
    }
    Ok(None)
}

fn parse_op(word: String) -> Result<u8, String> {
    if let Some(symbol) = word.chars().next() {
        for (c, b) in OP_DICTIONARY.iter() {
            if *c == symbol {
                let mut bits = *b;
                bits <<= 5;
                return Ok(bits);
            }
        }
    }
    Err("Error parsing op_code".to_string())
}

fn parse_reg(word: String) -> Result<u8, String> {
    if let Some(symbol) = word.chars().next() {
        for (c, b) in REG_DICTIONARY.iter() {
            if *c == symbol {
                return Ok(*b);
            }
        }
    }
    Err("Error parsing registry".to_string())
}

fn parse_r<I: Iterator<Item = String>>(mut line: I) -> Result<u8, String> {
    if let (Some(reg1), Some(reg2), Some(imm)) = (line.next(), line.next(), line.next()) {
        if let (Ok(mut reg1), Ok(mut reg2), Ok(imm)) =
            (parse_reg(reg1), parse_reg(reg2), imm.parse::<u8>())
        {
            reg1 <<= 3;
            reg2 <<= 1;
            let bits = reg1 + reg2 + imm;
            return Ok(bits);
        }
    }
    Err("Error parsing r-type instruction".to_string())
}

fn parse_i<I: Iterator<Item = String>>(mut line: I) -> Result<u8, String> {
    if let (Some(reg1), Some(imm)) = (line.next(), line.next()) {
        if let (Ok(mut reg1), Ok(imm)) = (parse_reg(reg1), imm.parse::<u8>()) {
            reg1 <<= 3;
            if imm > 7 {
                return Err("Error in i-type message, too large a value!".to_string());
            }
            let bits = reg1 + imm;
            return Ok(bits);
        }
    }
    Err("Error parsing i-type instruction".to_string())
}

fn parse_jump<I: Iterator<Item = String>>(
    mut line: I,
    labels: &mut Vec<(String, u32)>,
    pc: u32,
) -> Result<u8, String> {
    if let Some(s_code) = line.next() {
        let mut amount: i32 = 0;
        if let Ok(n) = s_code.parse() {
            amount = n;
        } else {
            for (label, place) in labels {
                if s_code == *label {
                    amount = *place as i32;
                    amount -= pc as i32;
                }
            }
        }
        if amount != 0 || amount > -16 || amount < 16 {
            if amount < 0 {
                return Ok((0b10000 + amount.abs()) as u8);
            } else {
                return Ok(amount as u8);
            }
        }
    }
    Err("Error parsing jump instruction".to_string())
}

fn parse_call<I: Iterator<Item = String>>(mut line: I) -> Result<u8, String> {
    if let Some(s_code) = line.next() {
        if let Some(symbol) = s_code.chars().next() {
            for (c, b) in SYSCALL_DICTIONARY.iter() {
                if *c == symbol {
                    return Ok(*b);
                }
            }
        }
        return Err("Error parsing syscall_symbol".to_string());
    }
    Err("Error parsing syscall instruction".to_string())
}
