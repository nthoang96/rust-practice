# The Rust Programming Language

_by Steve Klabnik and Carol Nichols, with contributions from the Rust Community_
Version: 1.67.1

---

## Getting started

### Installation

`rustup`

> a command line tool for managin Rust version and associated tools.

Updating
`rustup update`

### Hello-world

> Using a `!` means that you're calling a macro instead of a normal function and
> that macros don't always follow the same rules as funcitons.

#### Compiling and Running are separate steps

> Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc` command.
> After compiling successfully, Rust outputs a binary executable.

### Hello, Cargo!

Cargo is Rust's build system and package manager.

#### Create a Project with Cargo

`cargo new hello_cargo`

#### Building and Running a Cargo project

Build a project

`cargo build`

Build and run a project in one step

`cargo run`

Build project without producing a binary to check for errors

`cargo check`

> Instead of saving the result of the build in the same directory as our code, > Cargo stores it in the `target/debug` directory.

#### Building for Release

`cargo build --release`

> This command will create an executable in target/release instead of target/debug.

## Common Programming Concepts

### Variables and Mutability

> By default, variables are immutable

Although variables are immutable by default, you can make them mutable by adding `mut` in front of the variable name.

```rust
let mut x = 5;
```

#### Constants

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

- You aren't allowed to use `mut` with constants. (They're always immutable)
- You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated.
- Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

> `const` can be used in the global scope, and `let` can only be used in a function.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

> Rust’s naming convention for constants is to use all uppercase with underscores between words.

#### Shadowing

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

### Data Types

> Every value in Rust is of a certain data type.

#### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

##### Integer Types

An integer is a number without a fractional component.

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit |	i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

> Signed and unsigned refer to whether it's possible for the number to be negative--in other words, whether the number needs to have a sign with it or whether it will only ever be positive and can therefore be represented without a sign.

Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2^n - 1^ - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(2^7^) to 2^7^ - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2^n^ - 1, so a u8 can store numbers from 0 to 2^8^ - 1, which equals 0 to 255.

> Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as `arch`.

You can write integer literals in any of the forms shown in the table below.

| Number literals | Example |
| --------------- | ------- |
| Decimal |	98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte(u8 only) | b'A' |

Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`, which will have the same value as if you had specified `1000`.

**Integer Overflow**

Let’s say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error.

When you’re compiling in release mode with the `--release` flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
- Return the `None` value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.
