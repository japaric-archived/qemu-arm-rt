extern crate cc;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumbv") {
        let mut build = cc::Build::new();
        build.file("asm/syscall1.s").file("asm/syscall3.s");
        build.compile("asm");
    }

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_x = include_bytes!("link.x.in");
    let mut f = File::create(out.join("link.x")).unwrap();
    f.write_all(link_x).unwrap();
}
