#![no_std]

extern crate alloc;
use alloc::vec;
use linux_kernel_module;
use linux_kernel_module::println;

struct HelloWorldModule {
    array: vec::Vec<u64>,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello kernel module!");
        Ok(HelloWorldModule {
            array: vec![0;32],
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("Goodbye kernel module!");
    }
}

linux_kernel_module::kernel_module!(
    HelloWorldModule,
    license: "GPL"
);
