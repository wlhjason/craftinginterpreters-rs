use crate::memory::{free_array, grow_array, grow_capacity};
use crate::value::{Value, ValueArray};
use std::fmt;
use std::ptr::null_mut;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Constant),
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Subtract),
            3 => Ok(OpCode::Multiply),
            4 => Ok(OpCode::Divide),
            5 => Ok(OpCode::Negate),
            6 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{self:?}");
        write!(f, "OP_{}", name.to_uppercase())
    }
}

pub struct Chunk {
    pub count: usize,
    capacity: usize,
    pub code: *mut u8,
    pub lines: *mut usize,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            code: null_mut(),
            lines: null_mut(),
            constants: ValueArray::new(),
        }
    }

    pub unsafe fn write(&mut self, byte: u8, line: usize) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            unsafe {
                self.code = grow_array(self.code, old_capacity, self.capacity);
                self.lines = grow_array(self.lines, old_capacity, self.capacity);
            }
        }
        unsafe {
            *self.code.add(self.count) = byte;
            *self.lines.add(self.count) = line;
        }
        self.count += 1;
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        unsafe { self.constants.write(value) }
        self.constants.count - 1
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            free_array(self.code, self.capacity);
            free_array(self.lines, self.capacity);
        }
    }
}
