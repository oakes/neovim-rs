use std::path::Path;
use std::process::Command;
use std::env;

#[cfg(target_os="macos")]
fn print_lib_dir() {
    println!("cargo:rustc-flags=-L /usr/local/opt/gettext/lib/");
}

#[cfg(not(target_os="macos"))]
fn print_lib_dir() {
}

fn main() {
    let curr_dir_str = env::var("CARGO_MANIFEST_DIR").unwrap();
    let curr_dir = Path::new(curr_dir_str.as_slice());
    Command::new("git").arg("submodule").arg("update").arg("--init")
        .current_dir(&curr_dir).status().unwrap();

    let nvim_dir = curr_dir.join("neovim");
    Command::new("make").arg("deps").current_dir(&nvim_dir).status().unwrap();
    Command::new("make").arg("libnvim").current_dir(&nvim_dir).status().unwrap();

    let mut nvim_lib_dir = nvim_dir.clone();
    nvim_lib_dir.push("build");
    nvim_lib_dir.push("lib");
    let mut deps_lib_dir = nvim_dir.clone();
    deps_lib_dir.push(".deps");
    deps_lib_dir.push("usr");
    deps_lib_dir.push("lib");
    println!("cargo:rustc-flags=-L {} -L {} -l nvim:static",
        nvim_lib_dir.to_str().unwrap(),
        deps_lib_dir.to_str().unwrap());

    print_lib_dir();
}
