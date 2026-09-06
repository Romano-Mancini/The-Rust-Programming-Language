# Chapter 7 notes

Crates are the smallest amount of code the compiler can work with. They are

- binary crates: if they have a `main` function and are able to be executed on their own;
- library crates: when they only provide functionality with no `main` function.

Packages are simply bundles of crates which also contains a `Cargo.toml` file to describe how to build them.

So `cargo new` creates a `Cargo.toml` file, giving us a package.

So, `src/main.rs` is the first binary crate (conventionally, with the same name as the package), and eventually a `src/lib.rs` (same convention) would be the first library crate.
In `src/bin/` you can put the other binary crates. You can only have one library crate so no reason to search for that convention.
