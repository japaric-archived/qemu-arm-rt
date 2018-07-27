//! Core I/O functionality

use core::fmt::{self, Write};
use core::{slice, usize};

const EINTR: usize = 4;

/// A handle to the standard input stream of a process.
pub struct Stdin;

impl Stdin {
    /// Pull some bytes from this source into the specified buffer, returning how many bytes were
    /// read.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, usize> {
        const FD: usize = 0;

        let ec = unsafe { ::__syscall3(::nr::READ, FD, buffer.as_mut_ptr() as usize, buffer.len()) }
            as isize;

        if ec < 0 {
            Err(-ec as usize)
        } else {
            Ok(ec as usize)
        }
    }

    /// Read the exact number of bytes required to fill `buf`.
    pub fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), usize> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(EINTR) => {}
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty() {
            Err(usize::MAX)
        } else {
            Ok(())
        }
    }
}

const STDOUT_FD: usize = 1;

/// A handle to the global standard output stream of the current process.
pub struct Stdout;

impl Stdout {
    /// Attempts to write an entire buffer into this write.
    pub fn write_all(&mut self, buffer: &[u8]) -> Result<(), usize> {
        write_all(STDOUT_FD, buffer)
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_all(STDOUT_FD, s.as_bytes()).map_err(|_| fmt::Error)
    }
}

/// A handle to the standard error stream of a process.
pub struct Stderr;

impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        const FD: usize = 2;

        write_all(FD, s.as_bytes()).map_err(|_| fmt::Error)
    }
}

fn write_all(fd: usize, mut buffer: &[u8]) -> Result<(), usize> {
    while !buffer.is_empty() {
        let ec = unsafe {
            ::__syscall3(::nr::WRITE, fd, buffer.as_ptr() as usize, buffer.len()) as isize
        };

        if ec >= 0 {
            buffer = unsafe {
                slice::from_raw_parts(buffer.as_ptr().offset(ec), buffer.len() - ec as usize)
            }
        } else {
            let ec = -ec as usize;

            if ec == EINTR {
                continue;
            } else {
                return Err(ec);
            }
        }
    }

    Ok(())
}

#[doc(hidden)]
pub fn _print_str(s: &str) {
    Stdout.write_str(s).ok().expect("failed to print to stdout")
}

#[doc(hidden)]
pub fn _print_args(args: fmt::Arguments) {
    Stdout
        .write_fmt(args)
        .ok()
        .expect("failed to print to stdout")
}

#[doc(hidden)]
pub fn _eprint_str(s: &str) {
    Stderr.write_str(s).ok().expect("failed to print to stderr")
}

#[doc(hidden)]
pub fn _eprint_args(args: fmt::Arguments) {
    Stderr
        .write_fmt(args)
        .ok()
        .expect("failed to print to stderr")
}
