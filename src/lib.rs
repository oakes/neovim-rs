#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![allow(raw_pointer_deriving)]

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
    pub use libc::{c_char, c_int, int64_t, uint64_t};
    pub use libc::types::os::arch::c95::size_t;

    #[repr(C)]
    #[deriving(Copy)]
    pub struct C_msgpack_sbuffer {
        pub size: size_t,
        pub data: *const c_char,
        pub alloc: size_t,
    }
    #[repr(C)]
    #[deriving(Copy)]
    pub struct C_Object;
    #[repr(C)]
    #[deriving(Copy)]
    pub struct C_Array {
        pub items: *mut C_Object,
        pub size: size_t,
        pub capacity: size_t,
    }

    extern "C" {
        pub fn nvim_main (argc: c_int, argv: *const *const c_char,
                          read_fd: c_int, write_fd: c_int) -> c_int;
        pub fn vim_array_new () -> *mut C_Array;
        pub fn vim_array_free (arr: *mut C_Array);
        pub fn vim_array_add_int (val: int64_t, arr: *mut C_Array);
        pub fn vim_sbuffer_new () -> *mut C_msgpack_sbuffer;
        pub fn vim_sbuffer_free (sbuf: *mut C_msgpack_sbuffer);
        pub fn vim_serialize_request (request_id: uint64_t, method: *const c_char, args: *const C_Array, sbuf: *mut C_msgpack_sbuffer);
    }
}

#[deriving(Copy)]
pub struct Request {
    id: u64,
    method: &'static str,
    arguments: *mut ffi::C_Array,
}

impl Request {
    pub fn new(id: u64, method: &'static str) -> Request {
        Request {
            id: id,
            method: method,
            arguments: ::std::ptr::null_mut()
        }
    }

    fn check_arguments(&mut self) {
        if self.arguments.is_null() {
            self.arguments = unsafe { ffi::vim_array_new() };
        }
    }

    pub fn add_int(&mut self, val: i64) {
        self.check_arguments();
        unsafe { ffi::vim_array_add_int(val, self.arguments) }
    }

    pub fn serialize(&mut self) -> String {
        unsafe {
            let buf = ffi::vim_sbuffer_new();
            ffi::vim_serialize_request(self.id,
                                       self.method.to_c_str().as_ptr(),
                                       self.arguments as *const ffi::C_Array,
                                       buf);
            let s = String::from_raw_buf_len((*buf).data as *const u8, (*buf).size as uint);
            ffi::vim_sbuffer_free(buf);
            ffi::vim_array_free(self.arguments);
            self.arguments = ::std::ptr::null_mut();
            s
        }
    }
}

#[test]
fn test_request() {
    let mut r = Request::new(1, "attach_ui");
    r.add_int(80);
    r.add_int(24);
    println!("{}", r.serialize());
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
