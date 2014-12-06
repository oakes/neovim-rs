#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![feature(globs)]

extern crate libc;

mod ffi {
    use libc::{c_int, c_char};

    extern "C" {
        pub fn nvim_main (argc: c_int, argv: *mut *mut c_char);
    }
}

pub fn nvim_main() {
    unsafe {
        let mut args = ["nvim".to_c_str().as_mut_ptr()];
        ffi::nvim_main(args.len() as i32, args.as_mut_ptr());
    }
}
