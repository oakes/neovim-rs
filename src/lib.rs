#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]

extern crate libc;

use std::c_str::CString;
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
    pub use libc::{c_char, c_int, uint64_t};

    extern "C" {
        pub fn nvim_main (argc: c_int, argv: *const *const c_char) -> c_int;
        pub fn channel_from_fds (read_fd: c_int, write_fd: c_int) -> uint64_t;
        pub fn channel_subscribe (id: uint64_t, event: *const c_char);
    }
}

pub fn nvim_main(args: &[&str]) -> i32 {
    let v: Vec<CString> = args.iter().map(|s| s.to_c_str()).collect();
    let vp: Vec<*const ffi::c_char> = v.iter().map(|s| s.as_ptr()).collect();
    let p_vp: *const *const ffi::c_char = vp.as_ptr();

    unsafe { ffi::nvim_main(vp.len() as i32, p_vp) }
}

pub struct Channel {
    id: ffi::uint64_t
}

impl Channel {
    pub fn new(read_fd: i32, write_fd: i32) -> Channel {
        Channel {
            id: unsafe { ffi::channel_from_fds(read_fd, write_fd) }
        }
    }

    pub fn subscribe(&mut self, event: &str) {
        unsafe { ffi::channel_subscribe(self.id, event.to_c_str().as_ptr()) }
    }
}
