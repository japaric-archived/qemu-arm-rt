//! `panic!` works out of the box and matches the behavior of standard Rust programs

#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_arm_rt;

entry!(main);

fn main() {
    panic!();
}
