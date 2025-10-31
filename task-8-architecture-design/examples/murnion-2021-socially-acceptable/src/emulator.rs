use std::fs;
use rand::Rng;
use rand::rngs;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use chrono::prelude::*;

#[derive(Clone, PartialEq, Debug, Copy)]
enum Mood {
    Bored, 
    Happy, 
    Sick, 
    Maniacal, 
    Angry, 
    Annoyed, 
    Lovestruck, 
    Confused, 
}

#[derive(Clone, PartialEq, Debug, Copy)]
enum PrefixType {
    POLITE_STRONG,
    POLITE,
    DEMANDING,
    DEMANDING_STRONG,
}

impl PrefixType {
    fn find(code: [bool; 2]) -> PrefixType {
        match code {
            [false, false] => PrefixType::DEMANDING_STRONG,
            [false, true] => PrefixType::DEMANDING,
            [true, false] => PrefixType::POLITE,
            [true, true] => PrefixType::POLITE_STRONG,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Copy)]
enum OperationType {
    INCREMENT, 
    TO,
    ACCESS,
    LOOP,
    BRANCH_IF_GREATER,
    BRANCH_IF_ZERO,
    BRANCH_IF_EQUAL,
    JUMP,
}

impl OperationType {
    fn find(code: [bool; 3]) -> OperationType {
        match code {
            [false, false, false] => OperationType::INCREMENT,
            [false, false, true] => OperationType::TO,
            [false, true, false] => OperationType::ACCESS,
            [false, true, true] => OperationType::LOOP,
            [true, false, false] => OperationType::BRANCH_IF_GREATER,
            [true, false, true] => OperationType::BRANCH_IF_ZERO,
            [true, true, false] => OperationType::BRANCH_IF_EQUAL,
            [true, true, true] => OperationType::JUMP,
            _ => panic!()
        }
    }
}

// Generic helper function to convert compiler-fed code byte by byte into actual executable code.
fn convert_to_instruction(_byte: u8) -> (PrefixType, OperationType, [bool; 3]) {
    let mut byte = _byte;
    let base: u8 = 2;
    let mut bool_array: [bool; 8] = [false; 8];
    for i in 0..8 {
        if byte >= base.pow(7 - i) {
            byte -= base.pow(7 - i);
            bool_array[i as usize] = true;
        }
    }

    let prefix_type = PrefixType::find([bool_array[0], bool_array[1]]);
    let operation_type = OperationType::find([bool_array[2], bool_array[3], bool_array[4]]);
    let specifics = [bool_array[5], bool_array[6], bool_array[7]];

    (prefix_type, operation_type, specifics)
}

// Rng that only changes every hour.
fn generate_rng() -> rngs::StdRng {
    let time = chrono::offset::Utc::now();
    let hours = time.time().hour();
    let days = time.date().ordinal();
    let seed = hours * days;
    let mut rng: rngs::StdRng = rand::SeedableRng::seed_from_u64(seed as u64);
    rng
}

// Functionally just the "main" code, wrapped in a library.
pub fn emulate() {
    // Base values to get the emulator started.
    let mut irritation: i32 = 0;
    let mut last_was_positive = true;
    let mut social_credit: i32 = 0;
    let mut mood = Mood::Bored;
    let mut SMALL_TOLERANCE: i32 = 50;
    let mut SMALL_TOLERANCE_CLOSE: i32 = 25;
    let mut MEDIUM_TOLERANCE: i32 = 75;
    let mut MEDIUM_TOLERANCE_CLOSE: i32 = 50;
    let mut LARGE_TOLERANCE: i32 = 100;
    let mut LARGE_TOLERANCE_CLOSE: i32 = 75;

    // The speed at which different prefixes change your social credit and irritation.
    let mut polite_social_change: i32 = 2;
    let mut polite_strong_social_change: i32 = -5;
    let mut demanding_social_change: i32 = -2;
    let mut demanding_strong_social_change: i32 = -5;
    let mut irritation_change: i32 = 4;
    let mut irritation_decay: i32 = -1;

    // The technical assignments. Meant to emulate bespoke Assembly code for a custom-made chipset.
    let mut polite_registries: (i32, i32) = (0, 0);
    let mut demanding_registries: (i32, i32) = (0, 0);
    let mut stacks = (Vec::<i32>::new(), Vec::<i32>::new());
    let mut loops: (i32, i32) = (0, 0);
    let mut loop_counters: (i32, i32) = (0, 0);
    let mut loop_registries: (i32, i32) = (0, 0);
    let mut forcedmood: Option<u64> = None;

    // A lot of stuff for forcing the mood of the emulator to conform.
    // Almost necessary to be able to demonstrate the capabilities of the language.
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        match args[2].clone().as_str() {
            "--Bored" => forcedmood = Some(0),
            "--Happy" => forcedmood = Some(1),
            "--Sick" => forcedmood = Some(2),
            "--Maniacal" => forcedmood = Some(3),
            "--Angry" => forcedmood = Some(4),
            "--Annoyed" => forcedmood = Some(5),
            "--Lovestruck" => forcedmood = Some(6),
            "--Confused" => forcedmood = Some(7),
            _ => ()
        }
    }

    match if forcedmood.is_some() {forcedmood.unwrap()} else { generate_rng().gen::<u64>() % 8 } {
        0 => {
            mood = Mood::Bored;
        },
        1 => {
            mood = Mood::Happy;
            social_credit = 25;
            SMALL_TOLERANCE = 75;
            MEDIUM_TOLERANCE = 100;
            LARGE_TOLERANCE = 125;
            SMALL_TOLERANCE_CLOSE = 50;
            MEDIUM_TOLERANCE_CLOSE = 75;
            LARGE_TOLERANCE_CLOSE = 100;
        },
        2 => {
            mood = Mood::Sick;
            social_credit = -25;
            polite_strong_social_change = -7;
            demanding_social_change = -4;
            irritation_decay = 0;
        },
        3 => {
            mood = Mood::Maniacal;
            polite_social_change = 0;
            MEDIUM_TOLERANCE_CLOSE = 0;
        },
        4 => {
            mood = Mood::Angry;
            demanding_social_change = -4;
            demanding_strong_social_change = -8;
            polite_strong_social_change = -4;
            LARGE_TOLERANCE = 75;
            LARGE_TOLERANCE_CLOSE = 50;
        },
        5 => {
            mood = Mood::Annoyed;
            irritation_change = 8;
        },
        6 => {
            mood = Mood::Lovestruck;
            polite_social_change = 5;
            polite_strong_social_change = -2;
            social_credit = 25;
            SMALL_TOLERANCE = 100;
            MEDIUM_TOLERANCE = 125;
            LARGE_TOLERANCE = 150;
            SMALL_TOLERANCE_CLOSE = 50;
            MEDIUM_TOLERANCE_CLOSE = 75;
            LARGE_TOLERANCE_CLOSE = 100;
            irritation_change = 1;
        },
        7 => {
            mood = Mood::Confused;
            polite_social_change = -2;
            polite_strong_social_change = -4;
            demanding_social_change = 2;
            demanding_strong_social_change = 5;
        },
        _ => panic!("Generated a number outside of the range, for moods!")
    }

    // --get_mood option lets you see the mood of the emulator.
    if args.len() == 2 {
        if args[1].clone() == "get_mood" {
            println!("{:?}", mood);
            panic!("Got mood.");
        }
    }
    if args.len() < 2 {
        println!("[ERROR] A path to a SAL file must be provided!");
    } else {
        // Convert the read file (a byte list) into a list of operations.
        let commands: Vec<(PrefixType, OperationType, [bool; 3])> = fs::read(args[1].clone()).ok().unwrap()
                                                                                        .into_iter()
                                                                                        .map(|x| convert_to_instruction(x))
                                                                                        .collect();

        // Then loop through the commands.
        let mut i = 0;
        while i < commands.len() {
            let prefix_type = commands[i].0.clone();
            let operation_type = commands[i].1.clone();
            let specifics = commands[i].2;

            // Here we first check various social values, to determine if the operation is changed.
            if irritation >= 1000 {
                eprintln!("This program is DONE with your wishy-washy attitude.");
                std::thread::sleep(std::time::Duration::from_millis(500));
                panic!();
            }

            match prefix_type {
                PrefixType::POLITE => {
                    if social_credit >= SMALL_TOLERANCE || social_credit <= SMALL_TOLERANCE*-1 {
                        continue;
                    }
                    if social_credit >= SMALL_TOLERANCE_CLOSE || social_credit <= SMALL_TOLERANCE_CLOSE*-1 {
                        eprintln!("\"I guess...\"");
                    }
                    if !last_was_positive {
                        irritation += irritation_change;
                        last_was_positive = true;
                    } else if irritation > 0 {
                        irritation += irritation_decay;
                    }
                    social_credit += polite_social_change;
                },
                PrefixType::POLITE_STRONG => {
                    if social_credit >= LARGE_TOLERANCE || social_credit <= LARGE_TOLERANCE*-1 {
                        eprintln!("The program got tired of your snivelling attitude and left...");
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        panic!();
                    }
                    if social_credit >= LARGE_TOLERANCE_CLOSE || social_credit <= LARGE_TOLERANCE_CLOSE*-1 {
                        std::thread::sleep(std::time::Duration::from_millis(1500));
                    }
                    if last_was_positive {
                        irritation += irritation_change;
                        last_was_positive = false;
                    } else if irritation > 0 {
                        irritation += irritation_decay;
                    }
                    social_credit += polite_strong_social_change;
                },
                PrefixType::DEMANDING => {
                    if social_credit >= MEDIUM_TOLERANCE || social_credit <= MEDIUM_TOLERANCE*-1 {
                        eprintln!("\"Oh, now? Really, now?\"");
                        eprintln!("The program is defiantly doing nothing.");
                        std::thread::sleep(std::time::Duration::from_millis(15000));
                        eprintln!("\"Fine.\"");
                    }
                    if social_credit >= MEDIUM_TOLERANCE_CLOSE || social_credit <= MEDIUM_TOLERANCE_CLOSE*-1 {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                    if last_was_positive {
                        irritation += irritation_change;
                        last_was_positive = false;
                    } else if irritation > 0 {
                        irritation += irritation_decay;
                    }
                    social_credit += demanding_social_change;
                }
                PrefixType::DEMANDING_STRONG => {
                    if social_credit >= MEDIUM_TOLERANCE || social_credit <= MEDIUM_TOLERANCE*-1 {
                        eprintln!("\"No, you know what, f*** you!\"");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        panic!();
                    }
                    if last_was_positive {
                        irritation += irritation_change;
                        last_was_positive = false;
                    } else if irritation > 0 {
                        irritation += irritation_decay;
                    }
                    social_credit += demanding_strong_social_change;
                }
                _ => panic!()
            }

            // A lot of these are very ugly. Badly made, too.
            // Often, the operation will have to load the registry into a temporary value.
            // Then modify that value, and set the registry to that temporary value.
            match operation_type {
                OperationType::INCREMENT => {
                    let mut selected_registries: (i32, i32);
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        selected_registries = polite_registries;
                    } else {
                        selected_registries = demanding_registries;
                    }

                    let mut selected_registry = 0;
                    let mut other_registry = 0;
                    if specifics[0] {
                        selected_registry = selected_registries.1;
                        other_registry = selected_registries.0;
                    } else {
                        selected_registry = selected_registries.0;
                        other_registry = selected_registries.1;
                    }

                    let mut value = 1;
                    if specifics[2] {
                        value = other_registry;
                    }
                    if prefix_type == PrefixType::DEMANDING_STRONG && (social_credit >= MEDIUM_TOLERANCE_CLOSE || social_credit <= -1*MEDIUM_TOLERANCE_CLOSE) {
                        value *= 2;
                    }

                    if specifics[1] {
                        selected_registry -= value;
                    } else {
                        selected_registry += value;
                    }

                    if specifics[0] {
                        selected_registries.1 = selected_registry;
                    } else {
                        selected_registries.0 = selected_registry;
                    }

                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        polite_registries = selected_registries;
                    } else {
                        demanding_registries = selected_registries;
                    }
                },
                OperationType::TO => {
                    let mut selected_registries: (i32, i32);
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        selected_registries = polite_registries;
                    } else {
                        selected_registries = demanding_registries;
                    }

                    let mut selected_registry = 0;
                    let mut selected_stack: &mut Vec<i32>;
                    if specifics[0] {
                        selected_registry = selected_registries.1;
                        selected_stack = &mut stacks.1;
                    } else {
                        selected_registry = selected_registries.0;
                        selected_stack = &mut stacks.0;
                    }

                    if specifics[1] {
                        if specifics[2] {
                            let mut temp = selected_stack.pop();
                            if temp == None { temp = Some(0); }
                            selected_registry = temp.unwrap();
                        } else {
                            selected_registry /= 2;
                        }
                    } else {
                        if specifics[2] {
                            selected_stack.push(selected_registry);
                        } else {
                            selected_registry *= 2;
                        }
                    }

                    if specifics[0] {
                        selected_registries.1 = selected_registry;
                    } else {
                        selected_registries.0 = selected_registry;
                    }

                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        polite_registries = selected_registries;
                    } else {
                        demanding_registries = selected_registries;
                    }
                }
                OperationType::ACCESS => {
                    let mut selected_registries: (i32, i32);
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        selected_registries = polite_registries;
                    } else {
                        selected_registries = demanding_registries;
                    }

                    let mut selected_registry = 0;
                    if specifics[0] {
                        selected_registry = selected_registries.1;
                    } else {
                        selected_registry = selected_registries.0;
                    }

                    if !specifics[1] {
                        let mut input = String::new();
                        if specifics[2] {
                            let _input = std::io::stdin().read_line(&mut input);
                            if (input.chars().nth(0)).unwrap().is_ascii() {
                                selected_registry = ((input.chars().nth(0)).unwrap() as u8) as i32;
                            }
                        } else {
                            let _input = std::io::stdin().read_line(&mut input);
                            if (input.chars().nth(0)).unwrap().is_numeric() {
                                let temp = input.split_at(input.len() - 1).0.parse::<i32>();
                                if temp.is_ok() {
                                    selected_registry = temp.unwrap();
                                }
                            }
                        }
                    } else {
                        if specifics[2] {
                            if selected_registry < 256 {
                                println!("{:?}", (selected_registry as u8) as char)
                            }
                        } else {
                            println!("{}", selected_registry);
                        }
                    }

                    if specifics[0] {
                        selected_registries.1 = selected_registry;
                    } else {
                        selected_registries.0 = selected_registry;
                    }

                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        polite_registries = selected_registries;
                    } else {
                        demanding_registries = selected_registries;
                    }
                },
                OperationType::LOOP => {
                    let selected_registries: (i32, i32);
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        selected_registries = polite_registries;
                    } else {
                        selected_registries = demanding_registries;
                    }

                    let selected_registry;
                    if specifics[0] {
                        selected_registry = selected_registries.1;
                    } else {
                        selected_registry = selected_registries.0;
                    }

                    if specifics[2] {
                        if specifics[1] {
                            if loop_counters.1 < loop_registries.1 {
                                loop_counters.1 += 1;
                                i = loops.1 as usize;
                            }
                        } else {
                            loop_counters.1 = 0;
                            loop_registries.1 = selected_registry;
                            loops.1 = i as i32;
                        }
                    } else {
                        if specifics[1] {
                            if loop_counters.0 < loop_registries.0 {
                                loop_counters.0 += 1;
                                i = loops.0 as usize;
                            }
                        } else {
                            loop_registries.0 = selected_registry;
                            loops.0 = i as i32;
                            loop_counters.0 = 0;
                        }
                    }
                },
                OperationType::BRANCH_IF_GREATER => {
                    let mut selected_registries: (i32, i32);
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        selected_registries = polite_registries;
                    } else {
                        selected_registries = demanding_registries;
                    }

                    let mut selected_registry = 0;
                    let mut other_registry = 0;
                    if specifics[0] {
                        selected_registry = selected_registries.1;
                        other_registry = selected_registries.0;
                    } else {
                        selected_registry = selected_registries.0;
                        other_registry = selected_registries.1;
                    }

                    if selected_registry > other_registry {
                        let mut value = 1;
                        if specifics[1] {
                            value += 2;
                        }
                        if specifics[2] {
                            value += 1;
                        }
                        if prefix_type == PrefixType::DEMANDING_STRONG && (social_credit >= MEDIUM_TOLERANCE_CLOSE || social_credit <= -1*MEDIUM_TOLERANCE_CLOSE) {
                            value *= 2;
                        }
                        if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                            i += value;
                        } else {
                            i -= value;
                        }
                    }
                }
                OperationType::BRANCH_IF_ZERO => {
                    let mut selected_registries: (i32, i32);
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        selected_registries = polite_registries;
                    } else {
                        selected_registries = demanding_registries;
                    }

                    let mut selected_registry = 0;
                    if specifics[0] {
                        selected_registry = selected_registries.1;
                    } else {
                        selected_registry = selected_registries.0;
                    }

                    if selected_registry == 0 {
                        let mut value = 0;
                        if specifics[1] {
                            value += 2;
                        }
                        if specifics[2] {
                            value += 1;
                        }
                        if prefix_type == PrefixType::DEMANDING_STRONG && (social_credit >= MEDIUM_TOLERANCE_CLOSE || social_credit <= -1*MEDIUM_TOLERANCE_CLOSE) {
                            value *= 2;
                        }
                        if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                            i += value;
                        } else {
                            i -= value;
                        }
                    }
                }
                OperationType::BRANCH_IF_EQUAL => {
                    let selected_registries: (i32, i32);
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        selected_registries = polite_registries;
                    } else {
                        selected_registries = demanding_registries;
                    }

                    if selected_registries.0 == selected_registries.1 {
                        let mut value = 1;
                        if specifics[0] {
                            value += 4;
                        }
                        if specifics[1] {
                            value += 2;
                        }
                        if specifics[2] {
                            value += 1;
                        }
                        if prefix_type == PrefixType::DEMANDING_STRONG && (social_credit >= MEDIUM_TOLERANCE_CLOSE || social_credit <= -1*MEDIUM_TOLERANCE_CLOSE) {
                            value *= 2;
                        }
                        if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                            i += value;
                        } else {
                            i -= value;
                        }
                    }
                }
                OperationType::JUMP => {
                    let mut value = 1;
                    if specifics[0] {
                        value += 4;
                    }
                    if specifics[1] {
                        value += 2;
                    }
                    if specifics[2] {
                        value += 1;
                    }
                    if prefix_type == PrefixType::DEMANDING_STRONG && (social_credit >= MEDIUM_TOLERANCE_CLOSE || social_credit <= -1*MEDIUM_TOLERANCE_CLOSE) {
                        value *= 2;
                    }
                    if prefix_type == PrefixType::POLITE || prefix_type == PrefixType::POLITE_STRONG {
                        i += value;
                    } else {
                        i -= value;
                    }
                }
                _ => panic!(),
            }
            i += 1;
        }
    }
}