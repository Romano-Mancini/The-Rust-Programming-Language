# Chapter 3 notes

## Variables and mutability

Variables are by default immutable, unless you use `let mut` which makes them mutable and also conveys the idea that in the future they will be changing.

## Constants

You can use the keyword `const` to declare constants. What's the difference with an immutable variable?

1. You can declare them in any scope, even global scope. `let` can only be used in a function (e.g. `main`).;
2. You are forced to annotate their type;
3. Their value can only be an expression that can be evaluated at runtime.

## Shadowing

Differences between using `mut` and shadowing:

1. Shadowing allows you to change some values and then have it to be immutable;
2. Shadowing allows to change the type of the variable, since we are creating a new one from scratch that substitutes the previous one.

## Types

Rust has two kinds of types: scalar and compound. Scalar types represent only one value. Compound types group multiple values into one.

### Tuples

Tuples can't grow/shrink in size once declared: `let t : (i32, i64, isize) = (10, 20, 30);`. You can modify it if you declare it mutable (or with shadowing of course) with the syntax `t.0 = 11`. As before, if you don't use shadowing you still need to respect the type. The empty tuple `()` is called _unit_. Used to implicitly return nothing (similar to Haskell in this sense).

### Array

Every element of an array must be of the same type. They still have a fixed length.

## Statements vs Expressions

- Statements perform actions and do not return a value;
- Expressions evaluate into a value.

If it terminates with a semicolon, it's a statement. Else, it's an expression. You usually return the last expression from a function.
A function call is always considered to be an expression because if it does not return nothing it actually returns the unit `()`.
