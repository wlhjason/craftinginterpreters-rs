use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::vm::{free_vm, init_vm, interpret};

mod chunk;
mod debug;
mod memory;
mod value;
mod vm;

fn main() {
    unsafe {
        init_vm();

        let mut chunk = Chunk::new();

        let constant_index = chunk.add_constant(1.2);
        chunk.write(OpCode::Constant as u8, 123);
        chunk.write(constant_index as u8, 123);

        let constant_index = chunk.add_constant(3.4);
        chunk.write(OpCode::Constant as u8, 123);
        chunk.write(constant_index as u8, 123);

        chunk.write(OpCode::Add as u8, 123);

        let constant_index = chunk.add_constant(5.6);
        chunk.write(OpCode::Constant as u8, 123);
        chunk.write(constant_index as u8, 123);

        chunk.write(OpCode::Divide as u8, 123);

        chunk.write(OpCode::Negate as u8, 123);

        chunk.write(OpCode::Return as u8, 123);

        disassemble_chunk(&chunk, "test chunk");

        _ = interpret(&mut chunk);
        free_vm();
    };
}
