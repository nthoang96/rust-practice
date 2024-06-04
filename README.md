# The Rust Programming Language

_by Steve Klabnik and Carol Nichols, with contributions from the Rust Community_
Version: 1.67.1

---

1. Getting started

## Installation

`rustup`

> a command line tool for managin Rust version and associated tools.

Updating
`rustup update`

## Hello-world

> Using a `!` means that you're calling a macro instead of a normal function and
> that macros don't always follow the same rules as funcitons.

### Compiling and Running are separate steps

> Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc` command.
> After compiling successfully, Rust outputs a binary executable.

## Hello, Cargo!

Cargo is Rust's build system and package manager.

### Create a Project with Cargo

`cargo new hello_cargo`

### Building and Running a Cargo project

Build a project

`cargo build`

Build and run a project in one step

`cargo run`

Build project without producing a binary to check for errors

`cargo check`

> Instead of saving the result of the build in the same directory as our code, > Cargo stores it in the `target/debug` directory.

### Building for Release

`cargo build --release`

> This command will create an executable in target/release instead of target/debug.

## Common Programming Concepts

### Variables and Mutability

> By default, variables are immutable
