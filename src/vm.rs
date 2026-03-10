use crate::types::Instr;
use std::{
    collections::HashMap,
    io::{self, Write},
};

pub struct VM {
    stack: Vec<f64>,
    vars: HashMap<String, f64>,
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            vars: HashMap::new(),
            ip: 0,
        }
    }

    pub fn run(&mut self, code: Vec<Instr>) {
        while self.ip < code.len() {
            match &code[self.ip] {
                Instr::Push(n) => self.stack.push(*n),

                Instr::Load(v) => {
                    let val = *self.vars.get(v).unwrap_or(&0.0);
                    self.stack.push(val);
                }

                Instr::Store(v) => {
                    let val = self.stack.pop().unwrap();
                    self.vars.insert(v.clone(), val);
                }

                Instr::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }

                Instr::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }

                Instr::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }

                Instr::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }

                Instr::PrintText(msg) => {
                    println!("{}", msg);
                }

                Instr::PrintValue(msg) => {
                    let val = self.stack.pop().unwrap();
                    println!("{} {}", msg, val);
                }

                Instr::InputNumber(prompt, var) => {
                    print!("{}", prompt);
                    let _ = io::stdout().flush();
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).unwrap();

                    let v: f64 = buf.trim().parse().unwrap();
                    self.vars.insert(var.clone(), v);
                }

                Instr::Halt => break,
            }

            self.ip += 1;
        }
    }
}
