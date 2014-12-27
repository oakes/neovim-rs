#![allow(non_camel_case_types)]

pub use libc::{c_char, c_double, c_int, c_void, int64_t, uint64_t};
pub use libc::funcs::c95::stdlib::malloc;
pub use libc::types::os::arch::c95::size_t;

// misc types

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
pub struct C_KeyValuePair;

// object types

pub type C_Buffer = uint64_t;
pub type C_Window = uint64_t;
pub type C_Tabpage = uint64_t;
pub type C_Boolean = bool;
pub type C_Integer = int64_t;
pub type C_Float = c_double;

#[repr(C)]
#[deriving(Copy)]
pub struct C_String {
    pub data: *const c_char,
    pub size: size_t,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Array {
    pub items: *mut C_Object,
    pub size: size_t,
    pub capacity: size_t,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Dictionary {
    pub items: *mut C_KeyValuePair,
    pub size: size_t,
    pub capacity: size_t,
}

// functions

extern "C" {
    pub fn nvim_main (argc: c_int,
                      argv: *const *const c_char,
                      read_fd: c_int,
                      write_fd: c_int) -> c_int;
    pub fn api_free_array (arr: C_Array);
    pub fn vim_array_add_buffer (val: C_Buffer, arr: *mut C_Array);
    pub fn vim_array_add_window (val: C_Window, arr: *mut C_Array);
    pub fn vim_array_add_tabpage (val: C_Tabpage, arr: *mut C_Array);
    pub fn vim_array_add_nil (arr: *mut C_Array);
    pub fn vim_array_add_boolean (val: C_Boolean, arr: *mut C_Array);
    pub fn vim_array_add_integer (val: C_Integer, arr: *mut C_Array);
    pub fn vim_array_add_float (val: C_Float, arr: *mut C_Array);
    pub fn vim_array_add_string (val: C_String, arr: *mut C_Array);
    pub fn vim_array_add_array (val: C_Array, arr: *mut C_Array);
    pub fn vim_array_add_dictionary (val: C_Dictionary, arr: *mut C_Array);
    pub fn vim_msgpack_new () -> *mut C_msgpack_sbuffer;
    pub fn vim_msgpack_free (buf: *mut C_msgpack_sbuffer);
    pub fn vim_msgpack_parse (message: C_String, arr: *mut C_Array);
    pub fn vim_serialize_request (request_id: uint64_t,
                                  method: C_String,
                                  args: C_Array,
                                  buf: *mut C_msgpack_sbuffer);
}
