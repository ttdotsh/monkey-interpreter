# This project is a Rust implementation of the [Writing an Interpreter in Go book](https://interpreterbook.com/)

This repo is purely for learning the Rust language and about how interpreters/compilers work

## Instructions for running

Prerequisites:

* Rust installed
* Cargo installed

Run `cargo run --bin repl`

Write some Monkey code!

## Monkey syntax

At the moment, this implementation supports:
* variable bindings with `let` statements
* variables can be of type boolean, integer, or function
* higher order functions (functions that return other functions) and closures
* implicit returns
    * A block's last statement is implicitly returned
    * Monkey has a return keyword to support early returns
* if expressions
    * optional else blocks
    * if expressions can be used in variable bindings

A rundown of the syntax is as follows:

```
let five = 5;

five + 5;

let add = fn(x, y) { x + y };

let higher_order = fn() { fn(i) { i + five } };

let add_five = higher_order();

let thirteen = add_five(add_five(3));

let bool = if (thirteen > 10) { true } else { false };
```
