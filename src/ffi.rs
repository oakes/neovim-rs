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
pub struct C_KeyValuePair;

// object container types

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum ObjectType {
    BufferType,
    WindowType,
    TabpageType,
    NilType,
    BooleanType,
    IntegerType,
    FloatType,
    StringType,
    ArrayType,
    DictionaryType,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object {
    pub object_type: ObjectType,
    pub data: C_Array,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Buffer {
    pub object_type: ObjectType,
    pub data: C_Buffer,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Window {
    pub object_type: ObjectType,
    pub data: C_Window,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Tabpage {
    pub object_type: ObjectType,
    pub data: C_Tabpage,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Boolean {
    pub object_type: ObjectType,
    pub data: C_Boolean,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Integer {
    pub object_type: ObjectType,
    pub data: C_Integer,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Float {
    pub object_type: ObjectType,
    pub data: C_Float,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_String {
    pub object_type: ObjectType,
    pub data: C_String,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Array {
    pub object_type: ObjectType,
    pub data: C_Array,
}

#[repr(C)]
#[deriving(Copy)]
pub struct C_Object_Dictionary {
    pub object_type: ObjectType,
    pub data: C_Dictionary,
}

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
