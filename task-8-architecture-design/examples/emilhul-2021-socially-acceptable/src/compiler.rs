/***
 * Compiler for The Socially Acceptable Language
 * - handles compilation logic
 */
use std::collections::HashMap;

// Lengths of instructions and labels in bits
const INSTRUCTION_LENGTH: usize = 8;
const BRANCH_LENGTH: u32 = 2;
const JUMP_LENGTH: u32 = 3;

/// ## Prefix
/// The first two bits are represented by the Prefix. 
/// 
/// ### Types
/// There are four types:
/// * "I'M BEGGING YOU"
/// * "PLEASE"
/// * "NOW"
/// * "I ORDER YOU"
/// 
/// ### Groups
/// Prefixes can further be divided into two groups:
/// * Polite - "PLEASE" and "I'M BEGGING YOU"
/// * Demanding - "NOW" and "I ORDER YOU"
type Prefix = [bool; 2];
const ORDER: Prefix = [false, false];
const NOW: Prefix = [false, true];
const PLEASE: Prefix = [true, false];
const BEGGING: Prefix = [true, true];

/// Operation which uses three bits
type Operation = [bool; 3];
const INCREMENT: Operation = [false, false, false];
const TO: Operation = [false, false, true];
const ACCESS: Operation = [false, true, false];
const REPEAT: Operation = [false, true, true];
const BRANCH_IF_GREATER: Operation = [true, false, false];
const BRANCH_IF_ZERO: Operation = [true, false, true];
const JUMP_IF_EQUAL: Operation = [true, true, false];
const JUMP: Operation = [true, true, true];

// Registry uses one bit
type Registry = [bool; 1];
const FIRST: Registry = [false];
const SECOND: Registry = [true];

// Argument uses one bit
type Argument = [bool; 2];
const ONE: Argument = [false, false];
const TWO: Argument = [true, false];
const THIRD: Argument = [false, true];
const FOURTH: Argument = [true, true];

// There are four operation types
enum OperationType {
    OneArgument,
    TwoArguments,
    Branch,
    Jump,
}

/// Compile one SAL expression to an 8-bit instruction.
/// 
/// ### Operation types
/// | **Type** | **Encoding** |
/// |:---------|:-------------|
/// | One Argument | `Prefix<7:6>, Operation<5:3>, Registry<2>, Argument<1:0>` |
/// | Two Arguments | `Prefix<7:6>, Operation<5:3>, Registry<2>, First Argument<1>, Second Argument<0>` |
/// | Branch | `Prefix<7:6>, Operation<5:3>, Registry<2>, Label<1:0>` |
/// | Jump | `Prefix<7:6>, Opperation<5:3>, Label<2:0>` |
pub fn run(expression: &str, labels: &HashMap<String, usize>, current_index: &usize) -> Result<u8, usize> {
    // Split expression at , and .
    let components: Vec<&str> = expression.split(split_function).collect();
    let mut instruction: Vec<bool> = Vec::with_capacity(INSTRUCTION_LENGTH);

    // Check prefix and save wheter it's polite.
    let polite: bool = match get_prefix(components[0]) {
        Ok((_prefix, _polite)) => {
            instruction.extend_from_slice(&_prefix);
            _polite
        }
        Err(_) => return Err(1),
    };

    let operation_type: OperationType = match get_operation_and_type(components[1]) {
        Ok((_operation, _operation_type)) => {
            instruction.extend_from_slice(&_operation);
            _operation_type
        }
        Err(_) => return Err(2),
    };

    match operation_type {
        OperationType::OneArgument => {
            match get_registry(components[2]) {
                Ok(_registry) => instruction.extend_from_slice(&_registry),
                Err(_) => return Err(3),
            }
            match get_one_argument(components[3]) {
                Ok(_argument) => instruction.extend_from_slice(&_argument),
                Err(_) => return Err(4)
            }
        }
        OperationType::TwoArguments => {
            match get_registry(components[2]) {
                Ok(_registry) => instruction.extend_from_slice(&_registry),
                Err(_) => return Err(3),
            };
            match get_two_arguments(components[3], components[4]) {
                Ok(_argument) => instruction.extend_from_slice(&_argument),
                Err(_) => return Err(4),
            };
        }
        OperationType::Branch => {
            match get_registry(components[2]) {
                Ok(_registry) => instruction.extend_from_slice(&_registry),
                Err(_) => return Err(3),
            };
            match get_label(BRANCH_LENGTH, components[3], labels, current_index, polite) {
                Ok(_jump_distance) => {
                    let mut __jump_distance = _jump_distance;
                    for _i in (0..BRANCH_LENGTH).rev() {
                        let comp = 2u8.pow(_i as u32);
                        instruction.push(if __jump_distance >= comp {
                            __jump_distance -= comp;
                            true
                        } else {
                            false
                        });
                    }
                },
                Err(_) => return Err(5)
            }
        }
        OperationType::Jump => {
            match get_label(JUMP_LENGTH, components[2], labels, current_index, polite) {
                Ok(_jump_distance) => {
                    let mut __jump_distance = _jump_distance;
                    for _i in (0..JUMP_LENGTH).rev() {
                        let comp = 2u8.pow(_i as u32);
                        instruction.push(if __jump_distance >= comp {
                            __jump_distance -= comp;
                            true
                        } else {
                            false
                        });
                    }
                },
                Err(_) => return Err(5)
            }
        }
    };

    let mut executable_instruction: u8 = 0;
    for _i in 0..8 {
        if instruction[_i] {
            executable_instruction += 2u8.pow(7 - _i as u32);
        }
    }
    Ok(executable_instruction)
}

pub fn find_labels(expression: &str) -> Option<usize> {
    let start: Vec<&str> = expression.split_whitespace().collect();
    match expression.find(":"){
        Some(_index) if start[0] == "THE" => Some(_index),
        _ => None
    }
}

fn get_prefix(instruction: &str) -> Result<(Prefix, bool), usize> {
    match instruction {
        "I'M ORDERING YOU" | "I’M ORDERING YOU" => Ok((ORDER, false)),
        "NOW" => Ok((NOW, false)),
        "PLEASE" => Ok((PLEASE, true)),
        "I'M BEGGING YOU" | "I’M BEGGING YOU" => Ok((BEGGING, true)),
        _ => Err(1),
    }
}

fn get_operation_and_type(instruction: &str) -> Result<(Operation, OperationType), usize> {
    match instruction {
        " INCREMENT" => Ok((INCREMENT, OperationType::TwoArguments)),
        " TO" => Ok((TO, OperationType::OneArgument)),
        " ACCESS" => Ok((ACCESS, OperationType::TwoArguments)),
        " REPEAT THESE INSTRUCTIONS AN AMOUNT OF TIMES EQUAL TO" => Ok((REPEAT, OperationType::TwoArguments)),
        " IF THE SPECIFIED REGISTRY IS GREATER THAN THE UNSPECIFIED REGISTRY THEN JUMP TO THE SPECIFIED LABEL" => Ok((BRANCH_IF_GREATER, OperationType::Branch)),
        " IF THE SPECIFIED REGISTRY IS EQUAL TO ZERO JUMP TO THE SPECIFIED LABEL" => Ok((BRANCH_IF_ZERO, OperationType::Branch)),
        " IF THE REGISTRIES ARE EQUAL THEN JUMP TO" => Ok((JUMP_IF_EQUAL, OperationType::Jump)),
        " JUMP TO" => Ok((JUMP, OperationType::Jump)),
        _ => Err(2)
    }
}

fn get_registry(instruction: &str) -> Result<Registry, usize> {
    let mod_instruction = &instruction.replace(" COMPARING", "");
    match &mod_instruction[..] {
        " THE FIRST REGISTRY" => Ok(FIRST),
        " THE SECOND REGISTRY" => Ok(SECOND),
        _ => Err(3),
    }
}

fn get_one_argument(argument: &str) -> Result<Argument, usize> {
    match argument {
        " DOUBLE THE VALUE" => Ok(ONE),
        " HALVE THE VALUE" => Ok(TWO),
        " PUSH THE VALUE" => Ok(THIRD),
        " POP THE VALUE" => Ok(FOURTH),
        _ => return Err(4)
    }
}

fn get_two_arguments(first_argument: &str, second_argument: &str) -> Result<Argument, usize> {
    let first = match first_argument {
        " POSITIVELY" 
        | " INPUTTING A VALUE" 
        | " STARTING HERE" => false,
        " NEGATIVELY" 
        | " OUTPUTTING A VALUE" 
        | " ENDING HERE" => true,
        _ => return Err(4),
    };
    let second = match second_argument {
        " USING ONE" 
        | " AS AN INTEGER" 
        | " AND THIS IS THE FIRST LOOP" => false,
        " USING THE OTHER REGISTRY"
        | " AS A CHARACTER"
        | " AND THIS IS THE SECOND LOOP" => true,
        _ => return Err(4),
    };
    match (first, second) {
        (false, false) => Ok(ONE),
        (true, false) => Ok(TWO),
        (false, true) => Ok(THIRD),
        (true, true) => Ok(FOURTH),
    }
}

fn get_label(lenght: u32, instruction: &str, labels: &HashMap<String, usize>, current_index: &usize, polite: bool) -> Result<u8, usize> {
    match lenght {
        BRANCH_LENGTH => {
            let mod_instruction = &instruction.replace(" AND JUMPING TO ", "");
            let label_index = match labels.get(mod_instruction) {
                Some(_index) => _index,
                None => return Err(5)
                };
            let abs_diff = 
                if label_index < current_index && !polite {
                    current_index - label_index
                } else if label_index > current_index && polite {
                    label_index - current_index
                } else {
                    return Err(5)
                };
            if (abs_diff) <= 2usize.pow(lenght) {
                return Ok((abs_diff - 1) as u8);
            } else {
                return Err(5)
            }      
        }
        JUMP_LENGTH => {
            let mod_instruction = &instruction.replacen(" ", "", 1);
            let label_index = match labels.get(mod_instruction) {
            Some(_index) => _index,
            None => return Err(5)
            };
            let abs_diff = 
                if label_index < current_index && !polite {
                    current_index - label_index
                } else if label_index > current_index && polite {
                    label_index - current_index
                } else {
                    return Err(5)
                };
            if (abs_diff) <= 2usize.pow(lenght) {
                return Ok((abs_diff-1) as u8);
            } else {
                return Err(5)
            }
        }
        _ => return Err(5)
    }
}

fn split_function(c: char) -> bool {
    return c == ',' || c == '.';
}
