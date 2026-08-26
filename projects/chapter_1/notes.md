# Chapter 1 notes

## Different tools

- `rustup`: Installer for the Rust programming language
- `rustc`: Rust Compiler

## Macros

`println!()` is different from `println()`. The first one calls a macro, the second a function. The difference will be introduced later on. I see that only calling `println()` does not compile, I wonder why.

## Cargo

### Intro

Cargo is Rust's official build system and package manager. Will be useful to build complex projects and to integrate third party libraries.
If you use `cargo new hello_world` it will generate a package with a `cargo.toml` and a `src` directory with a `main.rs` file. It will also generate a git repository in case you are not in one already. With `cargo init` you can make a project use cargo easily.
Build the project with `cargo build`. The executable will be in `target/debug`. You can execute the binary separately.
To build and run directly, you can use `cargo run`. `cargo check` is a fast way to understand if the package compiles without actually spending time in compiling it.

### Profiles

There are two cargo profiles: when you usually do `cargo build` you use the development profile (that's why the binary is in a `debug` folder). This makes it faster to compile and test. When you do `cargo build --release` the build time is longer but more optimizations are applied, making the executable faster.
