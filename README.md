# The Rust Programming Language

_by Steve Klabnik and Carol Nichols, with contributions from the Rust Community_
Version: 1.67.1

---

1. Getting started

---

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

---

2. Common Programming Concepts

---

## Variables and Mutability

> By default, variables are immutable

Although variables are immutable by default, you can make them mutable by adding `mut` in front of the variable name.

```rust
let mut x = 5;
```

### Constants

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

- You aren't allowed to use `mut` with constants. (They're always immutable)
- You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated.
- Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

> `const` can be used in the global scope, and `let` can only be used in a function.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

> Rust’s naming convention for constants is to use all uppercase with underscores between words.

### Shadowing

In Rust, you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is *shadowed* by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

> We can shadow a variable by using the same variable's name and repeating the use of the `let` keyword.

> In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    // create a inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// the value of x in the inner scope is: 12
// the value of x is: 6
```

> We'll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.

> We’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

> Shadowing thus spares us from having to come up with different names, such as `spaces_str` and `spaces_num`; instead, we can reuse the simpler `spaces` name.

**If we try to use `mut` for this, as shown here, we'll get a compile-time error**

```rust
let mut spaces = "   ";
spaces = spaces.len();

// error: expected `&str`, found `usize`
```
