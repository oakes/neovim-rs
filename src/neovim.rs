#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![feature(globs)]

extern crate libc;

use std::str;

pub mod ffi;

pub fn path_tail(fname: &str) -> &str {
    unsafe { str::from_c_str(ffi::path_tail(fname.to_c_str().as_mut_ptr() as *mut u8) as *const i8) }
}
