//! Macros

/// See [`std::print!`](https://doc.rust-lang.org/std/macro.print.html)
#[macro_export]
macro_rules! print {
    ($s:expr) => ($crate::io::_print_str($s));
    ($($arg:tt)*) => ($crate::io::_print_args(format_args!($($arg)*)));
}

/// See [`std::println!`](https://doc.rust-lang.org/std/macro.println.html)
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($s:expr) => (print!(concat!($s, "\n")));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

/// See [`std::eprint!`](https://doc.rust-lang.org/std/macro.eprint.html)
#[macro_export]
macro_rules! eprint {
    ($s:expr) => ($crate::io::_eprint_str($s));
    ($($arg:tt)*) => ($crate::io::_eprint_args(format_args!($($arg)*)));
}

/// See [`std::eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html)
#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($s:expr) => (eprint!(concat!($s, "\n")));
    ($($arg:tt)*) => (eprint!("{}\n", format_args!($($arg)*)));
}

/// Macro to define the entry point of the program
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub extern "C" fn __impl_main() {
            // validate the signature of the program entry point
            let f: fn() = $path;

            f()
        }
    };
}
