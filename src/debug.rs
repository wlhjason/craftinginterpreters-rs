use crate::chunk::{Chunk, OpCode};

pub unsafe fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.count {
        unsafe { offset = disassemble_instruction(chunk, offset) }
    }
}

pub unsafe fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && unsafe { *chunk.lines.add(offset) == *chunk.lines.add(offset - 1) } {
        print!("   | ");
    } else {
        print!("{:4} ", unsafe { *chunk.lines.add(offset) })
    }

    let instruction = unsafe { *(chunk.code.add(offset)) };
    let opcode = OpCode::try_from(instruction).expect("Unknown opcode");
    let name = &format!("{}", opcode);
    match opcode {
        OpCode::Add
        | OpCode::Subtract
        | OpCode::Multiply
        | OpCode::Divide
        | OpCode::Negate
        | OpCode::Return => simple_instruction(name, offset),
        OpCode::Constant => constant_instruction(name, chunk, offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = unsafe { *chunk.code.add(offset + 1) };
    print!("{:-16} {:4} ", name, constant_index);
    let value = unsafe { *chunk.constants.values.add(constant_index as usize) };
    println!("'{}'", value);
    offset + 2
}
