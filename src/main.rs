use crate::compiler::compile;
use crate::vm::{InterpretError, free_vm, init_vm};
use std::io::{BufRead, Write};

mod chunk;
mod compiler;
mod debug;
mod memory;
mod scanner;
mod value;
mod vm;

fn main() {
    unsafe {
        init_vm();

        let args: Vec<String> = std::env::args().collect();
        match &args[..] {
            [_] => repl(),
            [_, path] => run_file(path),
            _ => {
                eprintln!("Usage: clox [path]");
                std::process::exit(64)
            }
        }

        free_vm();
    };
}

unsafe fn repl() {
    let mut line = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        match std::io::stdin().lock().read_line(&mut line).unwrap() {
            0 => {
                println!();
                break;
            }
            _ => unsafe { interpret(&line).unwrap() },
        }
    }
}

fn read_file(path: &str) -> String {
    let result = std::fs::read_to_string(path);
    match result {
        Ok(source) => source,
        Err(..) => {
            eprintln!("Could not open file {path}.");
            std::process::exit(74);
        }
    }
}

unsafe fn run_file(path: &str) {
    let source = read_file(path);

    let result = unsafe { interpret(&source) };
    if let Err(e) = result {
        std::process::exit(match e {
            InterpretError::CompileError => 65,
            InterpretError::RuntimeError => 70,
        })
    }
}

unsafe fn interpret(source: &str) -> Result<(), InterpretError> {
    unsafe {
        compile(source);
        Ok(())
    }
}
