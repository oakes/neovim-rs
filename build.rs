use std::io::Command;

fn main() {
    let cwd = Path::new("neovim");
    Command::new("make").cwd(&cwd).status().unwrap();
    let out_dir = "neovim/build/bin";
    println!("cargo:rustc-flags=-L {} -l nvim:static", out_dir);
}
