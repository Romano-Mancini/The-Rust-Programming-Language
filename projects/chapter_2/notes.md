# Chapter 2 notes

## Terms

The things that are automatically included in every Rust program are called "prelude". It's kept as small as possible, focusing on things that are inside each program like memory usage utilities. It is useful to have otherwise Rust would become too verbose in the imports.

## Variables

Variables are created via `let`. They are immutable. If you use `let mut`, they become mutable.
`String::new()` calls the associated function `new` from the type String, which is part of the prelude. It creates an empty string.

## Documentation

With `cargo doc --open` cargo opens in the browser the documentation for every crate in the project.

## Match

Each case expressed in a Match expression is called an arm.
The match expression ends after the first successful match, so it won’t look at each arm.
