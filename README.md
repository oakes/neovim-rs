## Introduction

A library for embedding Neovim inside a Rust project.

Code in the `examples` directory can be run with `cargo run --example <example>`.

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

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
