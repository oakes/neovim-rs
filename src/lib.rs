#![crate_name = "neovim"]
#![crate_type = "lib"]
#![crate_type = "rlib"]

extern crate libc;

#[cfg(target_os="macos")]
mod platform {
    #[link(name = "nvim")]
    #[link(name = "uv")]
    #[link(name = "msgpack")]
    #[link(name = "curses")]
    #[link(name = "iconv")]
    extern{}
}

#[doc(hidden)]
#[cfg(target_os="linux")]
mod platform {
    #[link(name = "nvim")]
    #[link(name = "uv")]
    #[link(name = "msgpack")]
    #[link(name = "curses")]
    extern{}
}

mod ffi {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn nvim_main (argc: c_int, argv: *const *const c_char) -> c_int;
    }
}

pub fn nvim_main() -> i32 {
    unsafe {
        let args = ["nvim".to_c_str().as_ptr()];
        ffi::nvim_main(args.len() as i32, args.as_ptr())
    }
}
