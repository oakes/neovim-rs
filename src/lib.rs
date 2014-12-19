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
        ffi::vim_serialize_request(id, vim_str, args.get_value(), buf);
        let s = String::from_raw_buf_len((*buf).data as *const u8, (*buf).size as uint);
        ffi::vim_msgpack_free(buf);
        s
    }
}

#[deriving(Copy)]
pub struct Array {
    value: ffi::C_Array
}

impl Array {
    pub fn new() -> Array {
        Array {
            value: ffi::C_Array {
                items: ::std::ptr::null_mut(),
                size: 0,
                capacity: 0,
            }
        }
    }

    pub fn add_buffer(&mut self, val: ffi::Buffer) {
        unsafe { ffi::vim_array_add_buffer(val, &mut self.value) }
    }

    pub fn add_window(&mut self, val: ffi::Window) {
        unsafe { ffi::vim_array_add_window(val, &mut self.value) }
    }

    pub fn add_tabpage(&mut self, val: ffi::Tabpage) {
        unsafe { ffi::vim_array_add_tabpage(val, &mut self.value) }
    }

    pub fn add_nil(&mut self) {
        unsafe { ffi::vim_array_add_nil(&mut self.value) }
    }

    pub fn add_boolean(&mut self, val: ffi::Boolean) {
        unsafe { ffi::vim_array_add_boolean(val, &mut self.value) }
    }

    pub fn add_integer(&mut self, val: ffi::Integer) {
        unsafe { ffi::vim_array_add_integer(val, &mut self.value) }
    }

    pub fn add_float(&mut self, val: ffi::Float) {
        unsafe { ffi::vim_array_add_float(val, &mut self.value) }
    }

    pub fn add_string(&mut self, val: &str) {
        unsafe {
            let vim_str = ffi::C_String {data: val.as_ptr() as *const i8, size: val.len() as u64};
            ffi::vim_array_add_string(vim_str, &mut self.value)
        }
    }

    pub fn add_array(&mut self, val: Array) {
        unsafe { ffi::vim_array_add_array(val.get_value(), &mut self.value) }
    }

    pub fn add_dictionary(&mut self, val: ffi::C_Dictionary) {
        unsafe { ffi::vim_array_add_dictionary(val, &mut self.value) }
    }

    pub fn len(&self) -> u64 {
        self.value.size
    }

    #[doc(hidden)]
    pub fn get_value(&self) -> ffi::C_Array {
        self.value
    }

    #[doc(hidden)]
    pub fn wrap_value(c_array: ffi::C_Array) -> Array {
        Array {
            value: c_array
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe { ffi::api_free_array(self.value) };
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
