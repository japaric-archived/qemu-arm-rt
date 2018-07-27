//! A module for working with processes

/// Terminates the current process with the specified exit code.
#[inline]
pub fn exit(code: i32) -> ! {
    unsafe {
        ::__syscall1(::nr::EXIT, code as usize);
        ::unreachable::unreachable()
    }
}
