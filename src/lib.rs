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

mod ffi;

pub fn run_with_fds(args: Vec<String>, read_fd: i32, write_fd: i32) -> i32 {
    let v: Vec<CString> = args.iter().map(|s| s.as_slice().to_c_str()).collect();
    let vp: Vec<*const ffi::c_char> = v.iter().map(|s| s.as_ptr()).collect();
    let p_vp: *const *const ffi::c_char = vp.as_ptr();

    unsafe { ffi::nvim_main(vp.len() as i32, p_vp, read_fd, write_fd) }
}

pub fn run(args: Vec<String>) -> i32 {
    run_with_fds(args, -1, -1)
}

pub fn serialize_request(id: u64, method: &'static str, args: &Array) -> String {
    unsafe {
        let buf = ffi::vim_msgpack_new();
        let vim_str = ffi::C_String {data: method.as_ptr() as *const i8, size: method.len() as u64};
        ffi::vim_serialize_request(id, vim_str, *args.get_pointer(), buf);
        let s = String::from_raw_buf_len((*buf).data as *const u8, (*buf).size as uint);
        ffi::vim_msgpack_free(buf);
        s
    }
}

#[deriving(Copy)]
pub struct Array {
    pointer: *mut ffi::C_Array
}

impl Array {
    pub fn new() -> Array {
        Array {
            pointer: ::std::ptr::null_mut()
        }
    }

    pub fn add_buffer(&mut self, val: ffi::Buffer) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_buffer(val, self.pointer) }
    }

    pub fn add_window(&mut self, val: ffi::Window) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_window(val, self.pointer) }
    }

    pub fn add_tabpage(&mut self, val: ffi::Tabpage) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_tabpage(val, self.pointer) }
    }

    pub fn add_nil(&mut self) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_nil(self.pointer) }
    }

    pub fn add_boolean(&mut self, val: ffi::Boolean) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_boolean(val, self.pointer) }
    }

    pub fn add_integer(&mut self, val: ffi::Integer) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_integer(val, self.pointer) }
    }

    pub fn add_float(&mut self, val: ffi::Float) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_float(val, self.pointer) }
    }

    pub fn add_string(&mut self, val: &str) {
        self.check_pointer();
        unsafe {
            let vim_str = ffi::C_String {data: val.as_ptr() as *const i8, size: val.len() as u64};
            ffi::vim_array_add_string(vim_str, self.pointer)
        }
    }

    pub fn add_array(&mut self, val: Array) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_array(*val.get_pointer(), self.pointer) }
    }

    pub fn add_dictionary(&mut self, val: ffi::C_Dictionary) {
        self.check_pointer();
        unsafe { ffi::vim_array_add_dictionary(val, self.pointer) }
    }

    pub fn clear(&mut self) {
        if !self.pointer.is_null() {
            unsafe { ffi::vim_array_free(self.pointer) };
        }
        self.pointer = ::std::ptr::null_mut();
    }

    pub fn len(&self) -> u64 {
        if !self.pointer.is_null() {
            unsafe { (*self.pointer).size }
        } else {
            0
        }
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *mut ffi::C_Array {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_array: *mut ffi::C_Array) -> Array {
        Array {
            pointer: c_array
        }
    }

    fn check_pointer(&mut self) {
        if self.pointer.is_null() {
            self.pointer = unsafe { ffi::vim_array_new() };
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        self.clear();
    }
}

#[test]
fn test_request() {
    let mut args = Array::new();
    args.add_integer(80);
    args.add_integer(24);
    args.add_string("hello");
    println!("{}", serialize_request(1, "attach_ui", &args));
}
