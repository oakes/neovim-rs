#[cfg(not(target_os="windows"))]
use std::process::Command;

use std::env;

#[cfg(target_os="macos")]
fn print_lib_dir() {
    println!("cargo:rustc-flags=-L /usr/local/opt/gettext/lib/");
}

#[cfg(not(target_os="macos"))]
fn print_lib_dir() {
}

#[cfg(target_os="windows")]
fn build_lib() {
    let curr_dir = env::current_dir().unwrap();
    let nvim_dir = curr_dir.join("neovim-windows");
    println!("cargo:rustc-flags=-L {}",
        nvim_dir.to_str().unwrap());
}

#[cfg(not(target_os="windows"))]
fn build_lib() {
    let curr_dir = env::current_dir().unwrap();
    let nvim_dir = curr_dir.join("neovim");
    Command::new("git").arg("submodule").arg("update").arg("--init")
        .current_dir(&curr_dir).status().unwrap();
    Command::new("make").arg("deps").current_dir(&nvim_dir).status().unwrap();
    Command::new("make").arg("libnvim").current_dir(&nvim_dir).status().unwrap();
    let nvim_lib_dir = nvim_dir.join("build").join("lib");
    let deps_lib_dir = nvim_dir.join(".deps").join("usr").join("lib");
    println!("cargo:rustc-flags=-L {} -L {}",
        nvim_lib_dir.to_str().unwrap(),
        deps_lib_dir.to_str().unwrap());
}

fn main() {
    build_lib();
    print_lib_dir();
}
