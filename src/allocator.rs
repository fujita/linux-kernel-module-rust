use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

use crate::bindings;
use crate::c_types;
use crate::println;

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // krealloc is used instead of kmalloc because kmalloc is an inline function and can't be
        // bound to as a result
        let ptr = bindings::krealloc(ptr::null(), layout.size(), bindings::GFP_KERNEL) as *mut u8;
        println!("kmalloc: addr={:p}, size={}", ptr, layout.size());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        println!("kfree: addr={:p}", ptr);
        bindings::kfree(ptr as *const c_types::c_void);
    }
}

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    panic!("Out of memory!");
}
