extern crate libc;
extern crate neovim;

use std::old_io::{File, FileMode, FileAccess};

mod ffi {
    pub use libc::{c_int, c_uchar, c_void};
    pub use libc::funcs::posix88::unistd::{close, pipe, read, write};
    pub use libc::types::os::arch::c95::size_t;
}

fn send_message(fd: ffi::c_int, command: &str) {
    let mut arr = neovim::Array::new();
    arr.add_string(command);
    let msg = neovim::serialize_message(1, "vim_command", &arr);
    let msg_ptr = msg.as_slice().as_ptr() as *const ffi::c_void;
    unsafe { ffi::write(fd, msg_ptr, msg.len() as ffi::size_t) };
}

fn recv_message(fd: ffi::c_int) -> Option<neovim::Array> {
    let mut buf : [ffi::c_uchar; 1024] = [0; 1024];
    let n = unsafe { ffi::read(fd, buf.as_mut_ptr() as *mut ffi::c_void, 1024) };
    if n < 0 {
        return None;
    }
    unsafe {
        let v = Vec::from_raw_buf(buf.as_ptr(), n as usize);
        let s = String::from_utf8_unchecked(v);
        Some(neovim::deserialize_message(&s))
    }
}

fn main() {
    // two pairs of anonymous pipes for msgpack-rpc between the logger and nvim
    let mut nvim_log : [ffi::c_int; 2] = [0; 2]; // to nvim from logger
    let mut log_nvim : [ffi::c_int; 2] = [0; 2]; // to logger from nvim
    unsafe {
        ffi::pipe(nvim_log.as_mut_ptr());
        ffi::pipe(log_nvim.as_mut_ptr());
    };

    // listen for events in a separate thread and log them
    ::std::thread::Thread::spawn(move || {
        // listen for bufenter events
        send_message(nvim_log[1], "au BufEnter * call rpcnotify(1, 'bufenter', fnamemodify(bufname(''), ':p'))");

        // receive messages
        let mut file = File::open_mode(&Path::new("events.log"), FileMode::Append, FileAccess::Write);
        while let Some(recv_arr) = recv_message(log_nvim[0]) {
            if recv_arr.len() > 0 {
                file.write_all(format!("{:?}\n", recv_arr).into_bytes().as_slice());
            }
        }
    });

    // start nvim
    let mut args = Vec::new();
    for arg in ::std::env::args() {
        args.push(arg);
    }
    neovim::main_setup(&args);
    neovim::channel_from_fds(nvim_log[0], log_nvim[1]);
    neovim::main_loop();
}
