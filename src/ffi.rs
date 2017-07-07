#![allow(non_camel_case_types)]

pub use libc::{c_char, c_double, c_int, c_void, int64_t, uint64_t, malloc, size_t};

// misc types

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_msgpack_sbuffer {
    pub size: size_t,
    pub data: *const c_char,
    pub alloc: size_t
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_KeyValuePair {
    pub key: C_String,
    pub value: C_Object
}

// object container types

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum ObjectType {
    NilType,
    BooleanType,
    IntegerType,
    FloatType,
    StringType,
    ArrayType,
    DictionaryType,
    BufferType,
    WindowType,
    TabpageType
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object {
    pub object_type: ObjectType,
    pub data: C_Array
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Buffer {
    pub object_type: ObjectType,
    pub data: C_Buffer
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Window {
    pub object_type: ObjectType,
    pub data: C_Window
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Tabpage {
    pub object_type: ObjectType,
    pub data: C_Tabpage
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Boolean {
    pub object_type: ObjectType,
    pub data: C_Boolean
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Integer {
    pub object_type: ObjectType,
    pub data: C_Integer
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Float {
    pub object_type: ObjectType,
    pub data: C_Float
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_String {
    pub object_type: ObjectType,
    pub data: C_String
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Array {
    pub object_type: ObjectType,
    pub data: C_Array
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Object_Dictionary {
    pub object_type: ObjectType,
    pub data: C_Dictionary
}

// object types

pub type C_Buffer = uint64_t;
pub type C_Window = uint64_t;
pub type C_Tabpage = uint64_t;
pub type C_Boolean = bool;
pub type C_Integer = int64_t;
pub type C_Float = c_double;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_String {
    pub data: *const c_char,
    pub size: size_t
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Array {
    pub items: *mut C_Object,
    pub size: size_t,
    pub capacity: size_t
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Dictionary {
    pub items: *mut C_KeyValuePair,
    pub size: size_t,
    pub capacity: size_t
}

// functions

extern "C" {
    pub fn channel_from_fds (read_fd: c_int, write_fd: c_int) -> uint64_t;
    pub fn nvim_main_setup (argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn nvim_main_loop () -> c_int;
    pub fn api_free_array (arr: C_Array);
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
