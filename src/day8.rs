use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use std::mem::replace;
fn perform_command(command: String) -> i32 {
    let this: Vec<&str> = command.split(" ").collect();
    match this[0] {
        "acc" => return this[1].parse::<i32>().unwrap(),
        _ => return 0,
    }
}
fn get_next_command(command: String, commands: Vec<String>, line_number: usize) -> (String, usize) {
    let this: Vec<&str> = command.split(" ").collect();
    match this[0] {
        "jmp" => {
            let change = this[1].parse::<i32>().unwrap();
            let new_linenumber: usize = (line_number as i32 + change) as usize;
            return (
                if new_linenumber >= commands.len() {
                    "".to_string()
                } else {
                    commands[new_linenumber].clone()
                },
                new_linenumber,
            );
        }
        _ => {
            let new_linenumber: usize = line_number + 1;
            return (
                if new_linenumber >= commands.len() {
                    "".to_string()
                } else {
                    commands[new_linenumber].clone()
                },
                new_linenumber,
            );
        }
    }
}
pub fn solve_a(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let lines: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    let mut line_number: usize = 0;
    let mut acc: i32 = 0;
    let mut command = lines[line_number].clone();
    let mut set: HashSet<usize> = HashSet::new();
    set.insert(0);
    loop {
        acc += perform_command(command.to_string());
        let result = get_next_command(command.to_string(), lines.clone(), line_number);
        command = result.0;
        if set.contains(&result.1) {
            break;
        } else {
            set.insert(result.1);
            line_number = result.1 as usize;
        }
    }
    println!("{}", acc)
}

fn check_infinite(commands: Vec<String>) -> (i32, bool) {
    let mut line_number: usize = 0;
    let mut acc: i32 = 0;
    let mut command = commands[line_number].clone();
    let mut set: HashSet<usize> = HashSet::new();
    set.insert(0);
    loop {
        acc += perform_command(command.to_string());
        let result = get_next_command(command.to_string(), commands.clone(), line_number);
        command = result.0;
        if set.contains(&result.1) {
            return (acc, false);
        } else if result.1 >= commands.len() {
            return (acc, true);
        } else {
            set.insert(result.1);
            line_number = result.1 as usize;
        }
    }
}
pub fn solve_b(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let lines: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    for (line_num, command) in lines.iter().enumerate() {
        let this: Vec<&str> = command.split(" ").collect();
        if this.contains(&"jmp") {
            // Change jmp to nop
            let new_lines: Vec<String> = lines
                .iter()
                .enumerate()
                .map(|(n, l)| {
                    if n == line_num {
                        l.replace("jmp", "nop")
                    } else {
                        l.to_string()
                    }
                })
                .collect();
            let (acc, pass) = check_infinite(new_lines);
            if pass {
                println!("{}", acc);
                return;
            }
        } else if this.contains(&"nop") {
            // Change nop to jmp
            let new_lines: Vec<String> = lines
                .iter()
                .enumerate()
                .map(|(n, l)| {
                    if n == line_num {
                        l.replace("nop", "jmp")
                    } else {
                        l.to_string()
                    }
                })
                .collect();
            let (acc, pass) = check_infinite(new_lines);
            if pass {
                println!("{}", acc);
                return;
            }
        }
    }
}
