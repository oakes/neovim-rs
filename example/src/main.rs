extern crate libc;
extern crate neovim;

use std::io::Write;
use std::fs::OpenOptions;
use std::path::Path;
use std::slice;
use libc::{c_int, c_uchar, c_void};
use libc::funcs::posix88::unistd::{pipe, read, write};
use libc::types::os::arch::c95::size_t;

fn send_message(fd: c_int, command: &str) {
    let mut arr = neovim::Array::new();
    arr.add_string(command);
    let msg = neovim::serialize_message(1, "vim_command", &arr);
    let msg_ref: &str = msg.as_ref();
    let msg_ptr = msg_ref.as_ptr() as *const c_void;
    unsafe { write(fd, msg_ptr, msg.len() as size_t) };
}

fn recv_message(fd: c_int) -> Option<neovim::Array> {
    let mut buf : [c_uchar; 1024] = [0; 1024];
    let n = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, 1024) };
    if n < 0 {
        return None;
    }
    unsafe {
        let v = slice::from_raw_parts(buf.as_ptr(), n as usize).to_vec();
        let s = String::from_utf8_unchecked(v);
        Some(neovim::deserialize_message(&s))
    }
}

fn main() {
    // two anonymous pipes for msgpack-rpc between the logger and nvim
    let mut nvim_log : [c_int; 2] = [0; 2]; // to nvim from logger
    let mut log_nvim : [c_int; 2] = [0; 2]; // to logger from nvim
    unsafe {
        pipe(nvim_log.as_mut_ptr());
        pipe(log_nvim.as_mut_ptr());
    };

    // open log file
    let mut opts = OpenOptions::new();
    opts.create(true);
    opts.write(true);
    opts.append(true);
    let mut file = opts.open(&Path::new("events.log")).unwrap();

    // listen for events in a separate thread and log them
    ::std::thread::spawn(move || {
        // listen for bufenter events
        send_message(nvim_log[1], "au BufEnter * call rpcnotify(1, 'bufenter', fnamemodify(bufname(''), ':p'))");

        // receive messages
        while let Some(recv_arr) = recv_message(log_nvim[0]) {
            if recv_arr.len() > 0 {
                let recv_str = format!("{:?}\n", recv_arr).into_bytes();
                file.write_all(recv_str.as_ref()).ok();
            }
        }
    });

    // start nvim
    let args : Vec<String> = ::std::env::args().collect();
    neovim::main_setup(&args);
    neovim::channel_from_fds(nvim_log[0], log_nvim[1]);
    neovim::main_loop();
}
