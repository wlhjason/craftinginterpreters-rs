use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
use crate::value::Value;
use std::ptr::null_mut;

const STACK_MAX: usize = 256;

pub struct VM {
    chunk: *mut Chunk,
    ip: *mut u8,
    stack: [Value; STACK_MAX],
    stack_top: *mut Value,
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

static mut VM: VM = VM {
    chunk: null_mut(),
    ip: null_mut(),
    stack: [0f64; STACK_MAX],
    stack_top: null_mut(),
};

pub unsafe fn reset_stack() {
    unsafe {
        VM.stack_top = &raw mut VM.stack[0];
    }
}
pub unsafe fn init_vm() {
    unsafe {
        reset_stack();
    }
}

pub unsafe fn free_vm() {}

pub unsafe fn interpret(source: &str) -> Result<(), InterpretError> {
    unsafe {
        compile(source);
        Ok(())
    }
}

unsafe fn push(value: Value) {
    unsafe {
        *VM.stack_top = value;
        VM.stack_top = VM.stack_top.add(1);
    }
}

unsafe fn pop() -> Value {
    unsafe {
        VM.stack_top = VM.stack_top.sub(1);
        *VM.stack_top
    }
}

unsafe fn read_byte() -> u8 {
    unsafe {
        let byte = *VM.ip;
        VM.ip = VM.ip.add(1);
        byte
    }
}

unsafe fn read_constant() -> Value {
    unsafe {
        let constant_index = read_byte() as usize;
        let values = (*VM.chunk).constants.values;
        *values.add(constant_index)
    }
}

#[macro_export]
macro_rules! binary_op {
    ( $op:tt ) => {
        {
            let b = pop();
            let a = pop();
            push(a $op b);
        }
    };
}

unsafe fn run() -> Result<(), InterpretError> {
    unsafe {
        loop {
            #[cfg(debug_assertions)]
            {
                use crate::debug::disassemble_instruction;

                print!("          ");
                let mut slot = &raw mut VM.stack[0];
                while VM.stack_top != slot {
                    print!("[ {} ]", *slot);
                    slot = slot.add(1);
                }
                println!();

                disassemble_instruction(&*VM.chunk, VM.ip.offset_from((*VM.chunk).code) as usize);
            }

            let instruction = read_byte();
            let opcode =
                OpCode::try_from(instruction).map_err(|()| InterpretError::CompileError)?;

            match opcode {
                OpCode::Constant => push(read_constant()),
                OpCode::Add => binary_op!(+),
                OpCode::Subtract => binary_op!(-),
                OpCode::Multiply => binary_op!(*),
                OpCode::Divide => binary_op!(/),
                OpCode::Negate => push(-pop()),
                OpCode::Return => {
                    let value = pop();
                    println!("'{}'", value);
                    return Ok(());
                }
            }
        }
    }
}
