use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;

mod chunk;
mod debug;
mod memory;
mod value;

fn main() {
    unsafe {
        let mut chunk = Chunk::new();

        let constant_index = chunk.add_constant(1.2);
        chunk.write(OpCode::Constant as u8, 123);
        chunk.write(constant_index as u8, 123);

        chunk.write(OpCode::Return as u8, 123);

        disassemble_chunk(&chunk, "test chunk");
    }
}
