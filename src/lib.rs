#![allow(raw_pointer_derive)]
#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]

extern crate libc;

use std::ffi::CString;
use std::fmt;
use std::slice;

#[cfg(target_os="macos")]
mod platform {
    #[link(name = "nvim", kind = "static")]
    #[link(name = "uv", kind = "static")]
    #[link(name = "msgpack", kind = "static")]
    #[link(name = "termkey", kind = "static")]
    #[link(name = "unibilium", kind = "static")]
    #[link(name = "vterm", kind = "static")]
    #[link(name = "util")]
    #[link(name = "intl")]
    #[link(name = "iconv")]
    extern{}
}

#[cfg(target_os="linux")]
mod platform {
    #[link(name = "nvim", kind = "static")]
    #[link(name = "uv", kind = "static")]
    #[link(name = "msgpack", kind = "static")]
    #[link(name = "termkey", kind = "static")]
    #[link(name = "unibilium", kind = "static")]
    #[link(name = "vterm", kind = "static")]
    #[link(name = "util")]
    extern{}
}

mod ffi;

pub enum Object {
    Buffer(ffi::C_Buffer),
    Window(ffi::C_Window),
    Tabpage(ffi::C_Tabpage),
    Boolean(ffi::C_Boolean),
    Integer(ffi::C_Integer),
    Float(ffi::C_Float),
    String(String),
    Array(Array),
    Dictionary(ffi::C_Dictionary)
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Buffer(ref obj) => write!(f, "Buffer({:?})", obj),
            Object::Window(ref obj) => write!(f, "Window({:?})", obj),
            Object::Tabpage(ref obj) => write!(f, "Tabpage({:?})", obj),
            Object::Boolean(ref obj) => write!(f, "Boolean({:?})", obj),
            Object::Integer(ref obj) => write!(f, "Integer({:?})", obj),
            Object::Float(ref obj) => write!(f, "Float({:?})", obj),
            Object::String(ref obj) => write!(f, "String({:?})", obj),
            Object::Array(ref arr) => {
                write!(f, "Array(").ok();
                let obj = arr.unwrap_value();
                for i in 0..obj.size {
                    let inner_obj_opt = unsafe { c_object_to_object(obj.items.offset(i as isize)) };
                    if let Some(inner_obj) = inner_obj_opt {
                        write!(f, "{:?}", inner_obj).ok();
                        if i + 1 < obj.size {
                            write!(f, ", ").ok();
                        }
                    } else {
                        write!(f, "Nil ").ok();
                    }
                }
                write!(f, ")")
            },
            Object::Dictionary(ref obj) => write!(f, "Dictionary(Length: {:?})", obj.size)
        }
    }
}

unsafe fn c_object_to_object(obj: *mut ffi::C_Object) -> Option<Object> {
    match (*obj).object_type {
        ffi::ObjectType::BufferType =>
            Some(Object::Buffer((*(obj as *mut ffi::C_Object_Buffer)).data)),
        ffi::ObjectType::WindowType =>
            Some(Object::Window((*(obj as *mut ffi::C_Object_Window)).data)),
        ffi::ObjectType::TabpageType =>
            Some(Object::Tabpage((*(obj as *mut ffi::C_Object_Tabpage)).data)),
        ffi::ObjectType::NilType =>
            None,
        ffi::ObjectType::BooleanType =>
            Some(Object::Boolean((*(obj as *mut ffi::C_Object_Boolean)).data)),
        ffi::ObjectType::IntegerType =>
            Some(Object::Integer((*(obj as *mut ffi::C_Object_Integer)).data)),
        ffi::ObjectType::FloatType =>
            Some(Object::Float((*(obj as *mut ffi::C_Object_Float)).data)),
        ffi::ObjectType::StringType => {
            let vim_str: ffi::C_String = (*(obj as *mut ffi::C_Object_String)).data;
            let v = slice::from_raw_parts(vim_str.data as *const u8, vim_str.size as usize).to_vec();
            Some(Object::String(String::from_utf8_unchecked(v)))
        },
        ffi::ObjectType::ArrayType =>
            Some(Object::Array(Array::wrap_value((*(obj as *mut ffi::C_Object_Array)).data))),
        ffi::ObjectType::DictionaryType =>
            Some(Object::Dictionary((*(obj as *mut ffi::C_Object_Dictionary)).data))
    }
}

pub fn main_setup(args: &Vec<String>) -> i32 {
    let args_vec_cstr: Vec<CString> = args.iter().map(|s| {
        let s_ref: &str = s.as_ref();
        CString::new(s_ref).unwrap()
    }).collect();
    let args_vec_ptr: Vec<*const ffi::c_char> = args_vec_cstr.iter().map(|s| s.as_ptr()).collect();
    unsafe { ffi::nvim_main_setup(args_vec_ptr.len() as i32, args_vec_ptr.as_ptr()) }
}

pub fn main_loop() -> i32 {
    unsafe { ffi::nvim_main_loop() }
}

pub fn channel_from_fds(read_fd: i32, write_fd: i32) -> u64 {
    unsafe { ffi::channel_from_fds(read_fd, write_fd) }
}

pub fn serialize_message(id: u64, method: &'static str, args: &Array) -> String {
    unsafe {
        let buf = ffi::vim_msgpack_new();
        let vim_str = ffi::C_String {data: method.as_ptr() as *const i8, size: method.len() as usize};
        ffi::vim_serialize_request(id, vim_str, args.unwrap_value(), buf);
        let v = slice::from_raw_parts((*buf).data as *const u8, (*buf).size as usize).to_vec();
        ffi::vim_msgpack_free(buf);
        String::from_utf8_unchecked(v)
    }
}

pub fn deserialize_message(message: &String) -> Array {
    let message_ref: &str = message.as_ref();
    let s = ffi::C_String {
        data: message_ref.as_ptr() as *const i8,
        size: message.len() as usize
    };
    let mut arr_raw = ffi::C_Array {
        items: ::std::ptr::null_mut(),
        size: 0,
        capacity: 0
    };
    unsafe { ffi::vim_msgpack_parse(s, &mut arr_raw) };
    Array::wrap_value(arr_raw)
}

pub struct Array {
    value: ffi::C_Array,
    is_owned: bool,
}

impl Array {
    pub fn new() -> Array {
        Array {
            value: ffi::C_Array {
                items: ::std::ptr::null_mut(),
                size: 0,
                capacity: 0
            },
            is_owned: true
        }
    }

    pub fn add_buffer(&mut self, val: ffi::C_Buffer) {
        unsafe { ffi::vim_array_add_buffer(val, &mut self.value) }
    }

    pub fn add_window(&mut self, val: ffi::C_Window) {
        unsafe { ffi::vim_array_add_window(val, &mut self.value) }
    }

    pub fn add_tabpage(&mut self, val: ffi::C_Tabpage) {
        unsafe { ffi::vim_array_add_tabpage(val, &mut self.value) }
    }

    pub fn add_nil(&mut self) {
        unsafe { ffi::vim_array_add_nil(&mut self.value) }
    }

    pub fn add_boolean(&mut self, val: ffi::C_Boolean) {
        unsafe { ffi::vim_array_add_boolean(val, &mut self.value) }
    }

    pub fn add_integer(&mut self, val: ffi::C_Integer) {
        unsafe { ffi::vim_array_add_integer(val, &mut self.value) }
    }

    pub fn add_float(&mut self, val: ffi::C_Float) {
        unsafe { ffi::vim_array_add_float(val, &mut self.value) }
    }

    pub fn add_string(&mut self, val: &str) {
        unsafe {
            // we need to copy the string into memory not managed by Rust,
            // so api_free_array can clear it
            let ptr = ffi::malloc(val.len() as usize);
            ::std::ptr::copy(val.as_ptr() as *const ffi::c_void, ptr, val.len());
            let vim_str = ffi::C_String {data: ptr as *const i8, size: val.len() as usize};
            ffi::vim_array_add_string(vim_str, &mut self.value)
        }
    }

    pub fn add_array(&mut self, val: &Array) {
        unsafe { ffi::vim_array_add_array(val.unwrap_value(), &mut self.value) }
    }

    pub fn add_dictionary(&mut self, val: ffi::C_Dictionary) {
        unsafe { ffi::vim_array_add_dictionary(val, &mut self.value) }
    }

    pub fn get(&self, index: usize) -> Option<Object> {
        if index >= self.len() {
            return None;
        }
        unsafe { c_object_to_object(self.value.items.offset(index as isize)) }
    }

    pub fn len(&self) -> usize {
        self.value.size
    }

    #[doc(hidden)]
    pub fn unwrap_value(&self) -> ffi::C_Array {
        self.value
    }

    #[doc(hidden)]
    pub fn wrap_value(c_array: ffi::C_Array) -> Array {
        Array {
            value: c_array,
            is_owned: false
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe { ffi::api_free_array(self.value) };
        }
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array(").ok();
        for i in 0..self.len() {
            if let Some(obj) = self.get(i) {
                write!(f, "{:?}", obj).ok();
            } else {
                write!(f, "Nil").ok();
            }
            if i + 1 < self.len() {
                write!(f, ", ").ok();
            }
        }
        write!(f, ")")
    }
}

#[test]
fn test_request() {
    let mut args = Array::new();
    args.add_integer(80);
    args.add_integer(24);
    args.add_string("hello");

    let msg = serialize_message(1, "test", &args);
    let arr = deserialize_message(&msg);
    println!("LENGTH: {}", arr.len());
    for i in 0..arr.len() {
        println!("{:?}", arr.get(i));
    }
}
