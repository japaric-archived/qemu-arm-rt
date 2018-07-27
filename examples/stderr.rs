//! Prints a message to STDERR and then exits the program with an exit code of 1

#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_arm_rt as rt;

use rt::process;

entry!(main);

fn main() {
    eprintln!("error: something went wrong");
    process::exit(1);
}
