extern crate regex;

use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
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
    left: bool
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
    let filename = "test.asm";
    let mut file = File::open(filename).unwrap();

    let mut deque: Deque = Deque {
        q: [].into(),
        left: false,
    };

    let mut instructions = String::new();
    file.read_to_string(&mut instructions).unwrap();
    instructions = instructions.replace("\r", "");
    instructions = instructions.replace("\t", "");

    let rg = Regex::new(r";.*").unwrap();
    instructions = rg.replace_all(instructions.as_str(), "").to_string();

    for (ip, line) in instructions.split("\n").enumerate() {
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
        let line = line.replacen(instruction, "", 1).replace(" ", "");
        let mut parameters = line.split(",");

        match instruction {
            "POP" => {
                deque.pop();
            },
            "PSH" => {
                parameters.for_each(|v| {
                    deque.push(v.parse().unwrap());
                });
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
                let x = deque.pop();
                let y = deque.pop();
                let z = deque.pop();
                deque.push(x);
                deque.push(z);
                deque.push(y);
            }
            "RCC" => {
                let x = deque.pop();
                let y = deque.pop();
                let z = deque.pop();
                deque.push(y);
                deque.push(x);
                deque.push(z);
            }
            "ROL" => {
                let back = deque.q
                    .pop_back()
                    .expect("{}: Stack underflow! Roll requires at least two values on the deque.");
                let front = deque.q
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

            "HLT" => {
                exit(0);
            }
            _ => {
                if instruction.ends_with(":") {
                    //is label
                } else {
                    panic!("{}: Command '{}' is invalid!", ip+1, instruction);
                }
            }
        }

        println!("{}: {:<3} -> {:?}", ip+1, instruction, deque.q);
    }
}
