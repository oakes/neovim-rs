use std::old_io::Command;
use std::env;

fn main() {
    let curr_dir = Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap());
    Command::new("git").arg("submodule").arg("update").arg("--init").cwd(&curr_dir).status().unwrap();

    let nvim_dir = curr_dir.join("neovim");
    Command::new("make").arg("deps").cwd(&nvim_dir).status().unwrap();
    Command::new("make").arg("libnvim").cwd(&nvim_dir).status().unwrap();

    let nvim_lib_dir = nvim_dir.join_many(&["build", "lib"]);
    let deps_lib_dir = nvim_dir.join_many(&[".deps", "usr", "lib"]);
    println!("cargo:rustc-flags=-L {} -L {} -l nvim:static",
        nvim_lib_dir.as_str().unwrap(),
        deps_lib_dir.as_str().unwrap());
}
