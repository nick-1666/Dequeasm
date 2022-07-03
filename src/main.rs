extern crate regex;

use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;
use std::process::exit;

trait DequeAbstraction {
    fn push(&mut self, value: u8);
    fn pop(&mut self) -> u8;
}

struct Deque {
    /// the queue
    q: VecDeque<u8>,
    /// internally set left boolean for
    /// operations on the front or back
    left: bool,
}
impl DequeAbstraction for Deque {
    fn push(&mut self, v: u8) {
        if self.left {
            self.q.push_front(v);
        } else {
            self.q.push_back(v);
        }
    }
    fn pop(&mut self) -> u8 {
        if self.left {
            self.q.pop_front().expect("Stack Underflow!")
        } else {
            self.q.pop_back().expect("Stack Underflow!")
        }
    }
}

fn main() {
    let debug = false;
    let filename = "test.asm";
    let mut file = File::open(filename).unwrap();

    let mut deque: Deque = Deque {
        q: [].into(),
        left: false,
    };

    let mut instructions = String::new();
    let rg = Regex::new("(;.*)*(\r)*(\t)*").unwrap();
    let rg_2 = Regex::new("\n\n").unwrap();

    file.read_to_string(&mut instructions).unwrap();
    instructions = rg.replace_all(instructions.as_str(), "").to_string();
    instructions = rg_2.replace_all(instructions.as_str(), "").to_string();
    let instructions = instructions
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut labels: HashMap<String, u8> = [].into();
    // pre-compute labels making sure there isn't any duplicates
    // Allows for jumping to unread labels

    for ip in 0..instructions.len() {
        let mut cmd: String = (&*instructions[ip]).to_string();
        if cmd.ends_with(":") {
            cmd.pop();
            if labels.contains_key(&*cmd) {
                panic!("{}: Label {:?} defined twice!", ip, cmd)
            }
            labels.insert(cmd, ip as u8);
        }
    }

    let mut ip= 0;

    while ip < instructions.len() {
        let line = &*instructions[ip];

        if line.len() == 0 {
            continue;
        }

        if line.starts_with("~") {
            deque.left = true;
        } else {
            deque.left = false;
        }
        let line = line.replace("~", "");
        let instruction = line.split_ascii_whitespace().next().unwrap();
        let line = line.replacen(instruction, "", 1);

        let seperator = Regex::new(r"[^\\](,)").unwrap();
        let parameters_pre_literals = seperator.split(&*line);
        let parameters: Vec<String> = parameters_pre_literals.map(|x| {
            return if x.trim().starts_with("\"") {
                x.to_string()
            } else {
                let o = x.trim().replace(" ", "").to_string();
                o
            }
        }).collect();

        match instruction {
            "POP" => {
                let x = parameters[0].parse::<i32>();
                if x.is_err() {
                    deque.pop();
                } else {
                    for _i in 0..x.unwrap() {
                        deque.pop();
                    }
                }
            }
            "PSH" => {
                for i in 0..parameters.len() {
                    let v = parameters.clone()[i].to_string();
                    if v.parse::<u8>().is_err() {
                        if v.trim().starts_with("\"") && v.ends_with("\"") {
                            let mut literal = false;

                            for c in v.trim()[1..v.trim().len()-1].chars() {
                                if c == '\\' {
                                    literal = true;
                                    continue;
                                }

                                if literal {
                                    match c {
                                        'n' => deque.push('\n' as u8),
                                        't' => deque.push('\t' as u8),
                                        '\"' => deque.push('\"' as u8),
                                        '\'' => deque.push('\'' as u8),
                                        '\\' => deque.push('\\' as u8),
                                        _ => {}
                                    }
                                    literal = false;
                                } else {
                                    deque.push(c as u8);
                                }
                            }
                        } else {
                            deque.push(*labels.get(&v).expect(&*format!("Label '{}' not found!", v)));
                        }
                    } else {
                        deque.push(v.parse::<u8>().unwrap());
                    }
                }
            }
            "DUP" => {
                let x = deque.pop();
                deque.push(x);
                deque.push(x);
            }
            "SWP" => {
                let x = deque.pop();
                let y = deque.pop();
                deque.push(y);
                deque.push(x);
            }
            "OVR" => {
                let x = deque.pop();
                let y = deque.pop();
                deque.push(y);
                deque.push(x);
                deque.push(y);
            }
            "RCW" => {
                let x = parameters[0].parse::<i32>();
                if x.is_err() {
                    let x = deque.pop();
                    let y = deque.pop();
                    let z = deque.pop();
                    deque.push(x);
                    deque.push(z);
                    deque.push(y);
                } else {
                    for _i in 0..x.unwrap() {
                        let x = deque.pop();
                        let y = deque.pop();
                        let z = deque.pop();
                        deque.push(x);
                        deque.push(z);
                        deque.push(y);
                    }
                }
            }
            "RCC" => {
                let x = parameters[0].parse::<i32>();
                if x.is_err() {
                    let x = deque.pop();
                    let y = deque.pop();
                    let z = deque.pop();
                    deque.push(y);
                    deque.push(x);
                    deque.push(z);
                } else {
                    for _i in 0..x.unwrap() {
                        let x = deque.pop();
                        let y = deque.pop();
                        let z = deque.pop();
                        deque.push(y);
                        deque.push(x);
                        deque.push(z);
                    }
                }
            }
            "SHR" => {
                let x = parameters[0].parse::<i32>();
                if x.is_err() {
                    deque.q.rotate_right(1);
                } else {
                    deque.q.rotate_right(x.unwrap() as usize);
                }
            }
            "SHL" => {
                let x = parameters[0].parse::<i32>();
                if x.is_err() {
                    deque.q.rotate_left(1);
                } else {
                    deque.q.rotate_left(x.unwrap() as usize);
                }
            }
            "ROL" => {
                let back = deque
                    .q
                    .pop_back()
                    .expect("{}: Stack underflow! Roll requires at least two values on the deque.");
                let front = deque
                    .q
                    .pop_front()
                    .expect("{}: Stack underflow! Roll requires at least two values on the deque.");
                deque.q.push_front(back);
                deque.q.push_back(front);
            }
            // Arithmetic Operations
            "ADD" => {
                let x = deque.pop();
                let y = deque.pop();
                deque.push(y + x);
            }
            "SUB" => {
                let x = deque.pop();
                let y = deque.pop();
                deque.push(y - x);
            }
            "MUL" => {
                let x = deque.pop();
                let y = deque.pop();
                deque.push(y * x);
            }
            "DIV" => {
                let x = deque.pop();
                let y = deque.pop();
                deque.push(y / x);
            }
            "MOD" => {
                let x = deque.pop();
                let y = deque.pop();
                deque.push(y % x);
            }
            "JMP" => {
                let x =  deque.pop() as usize;
                ip = x;
                if debug {
                    println!("{}: {:<3} -> {:?}", ip + 1, instruction, deque.q);
                }
                continue;
            }
            "JNZ" => {
                let x =  deque.pop() as usize;
                let y =  deque.pop() as usize;
                if y != 0 {
                    ip = x;
                    if debug {
                        println!("{}: {:<3} -> {:?}", ip + 1, instruction, deque.q);
                    }
                    continue;
                }
            }
            "INP" => {
                //WHY THE FUCK IS A CROSS COMPATIBLE GETCH SO HARD >:c
            }
            "OUT" => {
                let x = parameters[0].parse::<i32>();
                if x.is_err() {
                    let c = deque.pop() as char;
                    print!("{}", c);
                } else {
                    for _i in 0..x.unwrap() {
                        let c = deque.pop() as char;
                        print!("{}", c);
                    }
                }
            }
            "HLT" => {
                if debug {
                    println!("\n{:?}",  deque.q);
                }
                exit(0);
            }
            _ => {
                if !instruction.ends_with(":") {
                    panic!("{}: Unexpected command {:?}!\n*Hint: labels end with ':'", ip, instruction)
                } else {
                    ip += 1;
                    continue;
                }
            }
        }
        if debug {
            println!("{}: {:<3} -> {:?}", ip + 1,  instruction, deque.q);
        }
        ip += 1;
    }
    if debug {
        println!("\nEND: {:?}",  deque.q);
    }
    exit(0);
}
