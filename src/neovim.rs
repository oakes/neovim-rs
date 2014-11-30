#![crate_name = "neovim-rs"]
#![crate_type = "lib"]
#![crate_type = "rlib"]

extern crate libc;

//use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double, c_long, c_short, c_void, c_ulong};
use libc::{c_uchar};

extern "C" {
    pub fn path_tail (fname: *mut c_uchar) -> *mut c_uchar;
}
