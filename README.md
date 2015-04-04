## Introduction

A library for embedding Neovim inside a Rust project.

## Build Instructions

Note: Requires the nightly release of Rust due to the use of unstable features.

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
