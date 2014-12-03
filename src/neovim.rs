#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![feature(globs)]

extern crate libc;

pub mod ffi;

pub fn nvim_main() {
    unsafe {
        let mut args = ["nvim".to_c_str().as_mut_ptr()];
        ffi::nvim_main(args.len() as i32, args.as_mut_ptr());
    }
}
