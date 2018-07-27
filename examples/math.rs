//! Use a C implementation of libm to compute the sine of `1.5`

#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_arm_rt;

use core::{f32, ptr};

entry!(main);

fn main() {
    unsafe {
        #[link(name = "m")]
        extern "C" {
            fn sinf(x: f32) -> f32;
        }

        // This volatile round trip is to prevent LLVM from evaluating the sine at compile time
        let x = {
            static mut X: f32 = 0.;
            ptr::write_volatile(&mut X, 1.5);
            ptr::read_volatile(&X)
        };

        println!("{}", sinf(x));
    }
}
