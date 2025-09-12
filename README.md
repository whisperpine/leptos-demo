# Leptos Demo

A [leptos](https://github.com/leptos-rs/leptos) demo project.

## Prerequisites

### Nightly Rust

The preference of toolchain is specified in
[rust-toolchain.toml](./rust-toolchain.toml),\
so that your default rustup toolchain doesn't need to be changed.

However, the nightly toolchain should be install globally:

```sh
rustup toolchain install nightly
```

### WASM Target

Add build support for WebAssembly platform.

```sh
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Trunk

Use [Trunk](https://github.com/trunk-rs/trunk)
to build, bundle and ship Rust WASM application to the web.\
Install Trunk by either of the following ways
([binstall](https://github.com/cargo-bins/cargo-binstall) is worth mentioning):

```sh
cargo binstall trunk  # install trunk by downloading the binary
cargo install --locked trunk  # or install trunk from source
```

### Binaryen

[binaryen](https://github.com/WebAssembly/binaryen)
is used to reduce wasm file size after compilation.\
It must be installed in local dev environment (choose a preferred way to install).

The `wasm-opt` command  used in [./post-build.sh](./post-build.sh)
is from [binaryen](https://github.com/WebAssembly/binaryen).\
[Trunk](#trunk) will run [./post-build.sh](./post-build.sh)
in the post_build which is configured in [./Trunk.toml](./Trunk.toml).\
(albeit the post_build hook is disabled currently, cause it makes autoreload slow).

### leptosfmt

[leptosfmt](https://github.com/bram209/leptosfmt)
is not integral but highly recommended for quality of life reasons.

```sh
cargo binstall leptosfmt  # install leptosfmt by downloading the binary
cargo install --locked leptosfmt  # or install leptosfmt from source
```

## Getting Started

```sh
trunk serve  # build and serve locally with hot reload
trunk build --release  # build the release version
```
