//use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double, c_long, c_short, c_void, c_ulong};
use libc::{c_int, c_char};

extern "C" {
    pub fn nvim_main (argc: c_int, argv: *mut *mut c_char);
}
