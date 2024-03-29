//! rustix's `init` function.
//!
//! # Safety
//!
//! On mustang, or on any non-glibc non-musl platform, the `init` function must
//! be called before any other function in this module. It is unsafe because it
//! operates on raw pointers.
#![allow(unsafe_code)]

use crate::imp;

/// Initialize process-wide state.
///
/// # Safety
///
/// This must be passed a pointer to the original environment variable block
/// set up by the OS at process startup, and it must be called before any
/// other rustix functions are called.
#[inline]
#[doc(hidden)]
pub unsafe fn init(envp: *mut *mut u8) {
    imp::param::auxv::init(envp)
}
