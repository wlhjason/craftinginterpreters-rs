use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr::null_mut;

pub fn grow_capacity(capacity: usize) -> usize {
    if capacity < 8 { 8 } else { capacity * 2 }
}

pub unsafe fn grow_array<T>(ptr: *mut T, old_count: usize, new_count: usize) -> *mut T {
    unsafe { reallocate(ptr, old_count, new_count) }
}

pub unsafe fn free_array<T>(ptr: *mut T, old_count: usize) {
    unsafe {
        reallocate(ptr, old_count, 0);
    }
}

pub unsafe fn reallocate<T>(ptr: *mut T, _old_count: usize, new_count: usize) -> *mut T {
    let layout = Layout::new::<T>();
    let new_size = new_count * layout.size();
    unsafe {
        if new_size == 0 {
            System.dealloc(ptr.cast(), layout);
            null_mut()
        } else {
            let ptr = System.realloc(ptr.cast(), layout, new_size);
            if ptr.is_null() {
                panic!("Failed to reallocate memory")
            }
            ptr.cast()
        }
    }
}
