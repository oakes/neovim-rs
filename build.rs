use std::io::Command;
use std::os;

fn main() {
    let curr_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    Command::new("git").arg("submodule").arg("update").arg("--init").cwd(&curr_dir).status().unwrap();

    let nvim_dir = curr_dir.join("neovim");
    Command::new("make").arg("libnvim").cwd(&nvim_dir).status().unwrap();

    let nvim_lib_dir = nvim_dir.join_many(&["build", "lib"]);
    let deps_lib_dir = nvim_dir.join_many(&[".deps", "usr", "lib"]);
    println!("cargo:rustc-flags=-L {} -L {} -l nvim:static",
        nvim_lib_dir.as_str().unwrap(),
        deps_lib_dir.as_str().unwrap());
}
