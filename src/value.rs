use crate::memory::{free_array, grow_array, grow_capacity};

pub type Value = f64;

pub struct ValueArray {
    pub count: usize,
    capacity: usize,
    pub values: *mut Value,
}

impl ValueArray {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            values: std::ptr::null_mut(),
        }
    }

    pub unsafe fn write(&mut self, value: Value) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            unsafe { self.values = grow_array(self.values, old_capacity, self.capacity) }
        }
        unsafe { *self.values.add(self.count) = value }
        self.count += 1;
    }
}

impl Drop for ValueArray {
    fn drop(&mut self) {
        unsafe { free_array(self.values, self.capacity) }
    }
}
