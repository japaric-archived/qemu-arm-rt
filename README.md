# Status

This crate is an old experiment and has been archived. In general, you should
prefer QEMU system emulation to run Cortex-M programs. Check the [embedded Rust
book] to learn how to run Rust program on qemu-system-arm.

[embedded Rust book]: https://docs.rust-embedded.org/book

-- @japaric, 2018-12-08

---

# `qemu-arm-rt`

> Minimal runtime for emulation of Cortex-M programs

## What is this?

This is a *runtime crate* in the vein of [`cortex-m-rt`] that lets you build `#![no_std]` binaries
for the `thumbv*m-none-eabi*` targets. These binaries can then be executed on the host using QEMU
user emulation. Since the binary runs on a hosted environment (e.g. Linux) the runtime crate
also provides an API to do standard I/O on the host (`io::Stdin`, `io::Stdout`, `io::Stderr` and the
whole family of `print!` macros).

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

``` console
$ cat examples/hello.rs
```

``` rust
//! The family of `print!` macros Just Work

#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_arm_rt;

entry!(main);

fn main() {
    println!("Hello, world!");
}
```

``` console
$ cargo run --example hello --target thumbv7m-none-eabi
     Running `qemu-arm -cpu cortex-m3 target/thumbv7m-none-eabi/debug/examples/hello`
Hello, world!

$ arm-none-eabi-readelf -h target/thumbv7m-none-eabi/debug/examples/hello | head
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
```

It should be noted that these binaries lack any meaningful memory layout and can't be flashed on
actual microcontrollers.

## Why did you make this?

I wanted to compare [the Rust port of MUSL's libm][libm] to newlib's libm but, for reasons beyond
me, it's not possible to compile newlib natively (i.e. for a Linux host). I already had newlib
compiled for different ARM Cortex-M processors on my disk (from the `arm-none-eabi-newlib` package)
so I figured I might as well use that and avoid the pain of compiling C code and dealing with C
build systems. The result is this crate; with it I can write Rust programs that link to that
pre-compiled libm library and run them pseudo-natively using QEMU.

[libm]: https://github.com/japaric/libm

QEMU user emulation is more like a translation layer rather than a properly emulated core so most of
the operations in [`core::arch::arm`], like `__set_BASEPRI` and `__WFI`, and [`cortex-m`] will cause
QEMU to crash. QEMU system emulation is much better in that regard: most of the operations in
`core::arch::arm` will work, or at least they won't crash QEMU. However, this crate is not
appropriate for QEMU system emulation as system calls, which are widely used in the `io` and
`process` APIs, have no meaning in an emulated Cortex-M core. If you are interested in getting QEMU
system emulation working with Cortex-M binaries check the [`lm3s6965evb`] repository.

[`core::arch::arm`]: https://doc.rust-lang.org/core/arch/arm/index.html
[`cortex-m`]: https://crates.io/crates/cortex-m
[`lm3s6965evb`]: https://github.com/japaric/lm3s6965evb

## How to use this crate?

To use this crate to build a Cortex-M binary that can be emulated using QEMU user emulation follow
these steps:

- Install the QEMU user emulation binary if you haven't already. It should be named `qemu-arm`. On
  Arch Linux, this binary was provided by the `qemu-arch-extra` package.

- Create a new Cargo project

``` console
$ cargo new --bin app && cd $_
```

- Add this crate as a dependency

``` console
$ cargo add qemu-arm-rt --git https://github.com/japaric/qemu-arm-rt
```

- Write a Cargo configuration file. This file will tweak the linking process.

``` console
$ mkdir .cargo

$ curl -L https://raw.githubusercontent.com/japaric/qemu-arm-rt/master/.cargo/config > .cargo/config
```

- Get some starter code

``` console
$ curl -L https://raw.githubusercontent.com/japaric/qemu-arm-rt/master/examples/hello.rs > src/main.rs
```

- Build and run the program

``` console
$ # build the program for a Cortex-M3 microcontroller
$ cargo run --target thumbv7m-none-eabi
     Running `qemu-arm -cpu cortex-m3 target/thumbv7m-none-eabi/debug/app`
Hello, world!
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
