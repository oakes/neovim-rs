#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]

extern crate libc;

use std::c_str::CString;
use std::string::String;
use std::vec::Vec;

#[cfg(target_os="macos")]
mod platform {
    #[link(name = "nvim")]
    #[link(name = "uv")]
    #[link(name = "msgpack")]
    #[link(name = "curses")]
    #[link(name = "iconv")]
    extern{}
}

#[cfg(target_os="linux")]
mod platform {
    #[link(name = "nvim")]
    #[link(name = "uv")]
    #[link(name = "msgpack")]
    #[link(name = "curses")]
    extern{}
}

#[cfg(target_os="windows")]
mod platform {
    #[link(name = "nvim")]
    #[link(name = "uv")]
    #[link(name = "msgpack")]
    #[link(name = "curses")]
    extern{}
}

mod ffi {
    pub use libc::{c_char, c_int};

    extern "C" {
        pub fn nvim_main (argc: c_int, argv: *const *const c_char,
                          read_fd: c_int,
                          write_fd: c_int) -> c_int;
    }
}

pub fn run_with_fds(args: Vec<String>, read_fd: i32, write_fd: i32) -> i32 {
    let v: Vec<CString> = args.iter().map(|s| s.as_slice().to_c_str()).collect();
    let vp: Vec<*const ffi::c_char> = v.iter().map(|s| s.as_ptr()).collect();
    let p_vp: *const *const ffi::c_char = vp.as_ptr();

    unsafe { ffi::nvim_main(vp.len() as i32, p_vp, read_fd, write_fd) }
}

pub fn run(args: Vec<String>) -> i32 {
    run_with_fds(args, -1, -1)
}
