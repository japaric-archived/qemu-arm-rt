//! Minimal runtime for emulation of Cortex-M programs

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(panic_implementation)]
#![no_std]

extern crate unreachable;

use core::panic::PanicInfo;

#[macro_use]
pub mod macros;
pub mod io;
pub mod process;

mod nr;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn Reset() {
    extern "C" {
        fn main();
    }

    unsafe { main() }

    process::exit(0)
}

// Because this program will be executed within QEMU's user emulation mode we can use system calls!
extern "C" {
    fn __syscall1(nr: usize, a0: usize) -> usize;
    fn __syscall3(nr: usize, a0: usize, a1: usize, a2: usize) -> usize;
}

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("process {}", info);
    process::exit(134);
}
