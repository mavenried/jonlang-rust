use std::{env, fs};

mod compiler;
mod parser;
mod tokenizer;
mod types;
mod vm;

use compiler::*;
use parser::*;
use tokenizer::*;
use types::Instr;
use vm::*;

fn split_lines(src: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut buf = String::new();
    let mut quote = false;

    for c in src.chars() {
        if c == '"' {
            quote = !quote;
            buf.push(c);
        } else if c == '!' && !quote {
            lines.push(buf.trim().to_string());
            buf.clear();
        } else {
            buf.push(c);
        }
    }

    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let src = fs::read_to_string(&args[1]).unwrap();
    let lines = split_lines(&src);

    let mut code = Vec::new();
    let mut jon_awake = false;

    for line in lines {
        if line == "hi, jon" {
            jon_awake = true;
            continue;
        }

        if line == "bye, jon" {
            jon_awake = false;
            continue;
        }

        if !jon_awake {
            continue;
        }

        let tokens = tokenize(&line);
        let stmt = parse(tokens);

        compile_stmt(stmt, &mut code);
    }

    code.push(Instr::Halt);

    if args.contains(&"--debug".to_string()) {
        println!("\x1b[33m::Bytecode::");
        for line in &code {
            println!("{line:?}")
        }
        println!("::--------::\n\x1b[0m");
    }

    let mut vm = VM::new();
    vm.run(code);
}
