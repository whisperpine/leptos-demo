# README

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
cargo install --locked trunk  # install trunk from source
```

## Getting Started

```sh
trunk serve  # build and serve locally with hot reload
trunk build --release  # build the release version
```
