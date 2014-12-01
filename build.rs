use std::io::Command;
use std::os;

fn main() {
    let curr_dir = os::getenv("CARGO_MANIFEST_DIR").unwrap();
    let nvim_dir = Path::new(curr_dir).join("neovim");
    Command::new("make").arg("libnvim").cwd(&nvim_dir).status().unwrap();

    let nvim_lib_dir = nvim_dir.join_many(&["build", "src", "nvim"]);
    let deps_lib_dir = nvim_dir.join_many(&[".deps", "usr", "lib"]);
    println!("cargo:rustc-flags=-L {} -L {} -l nvim:static -l nvim -l uv -l msgpack -l curses",
        nvim_lib_dir.as_str().unwrap(),
        deps_lib_dir.as_str().unwrap());
}
