## Introduction

A library for embedding Neovim inside a Rust project.

The example can be run with `cargo run --example events`. It will launch neovim, and every time you open a buffer it will save the event to `events.log`.

## Build Instructions

Note: If neovim fails to build, try going into the `neovim` directory and running `make libnvim` to get more specific errors.

### Linux (apt-get)

```Shell
apt-get install libtool autoconf automake cmake libncurses5-dev g++ pkg-config unzip
cargo build
```

### Linux (yum)

```Shell
yum install autoconf automake cmake gcc gcc-c++ libtool ncurses-devel pkgconfig
cargo build
```

### OS X (homebrew)

```Shell
brew install libtool automake cmake pkg-config gettext
cargo build
```

## Windows

Prebuilt binaries are already in the `neovim-windows` dir, so neovim will not need to be built. You will, however, need to install the `rustup install stable-gnu`.

```Shell
set RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
cargo build
```

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
