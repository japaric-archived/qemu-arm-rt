//! The family of `print!` macros Just Work

#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_arm_rt;

entry!(main);

fn main() {
    println!("Hello, world!");
}
