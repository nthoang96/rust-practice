# The Rust Programming Language

<!--toc:start-->

- [The Rust Programming Language](#the-rust-programming-language)
  - [Getting started](#getting-started)
    - [Installation](#installation)
    - [Hello-world](#hello-world)
      - [Compiling and Running are separate steps](#compiling-and-running-are-separate-steps)
    - [Hello, Cargo!](#hello-cargo)
      - [Create a Project with Cargo](#create-a-project-with-cargo)
      - [Building and Running a Cargo project](#building-and-running-a-cargo-project)
      - [Building for Release](#building-for-release)
  - [Common Programming Concepts](#common-programming-concepts) - [Variables and Mutability](#variables-and-mutability) - [Constants](#constants) - [Shadowing](#shadowing) - [Data Types](#data-types) - [Scalar Types](#scalar-types) - [Integer Types](#integer-types) - [Floating-Point Types](#floating-point-types) - [Numeric Operations](#numeric-operations) - [The Boolean Type](#the-boolean-type) - [The Character Type](#the-character-type) - [Compound Types](#compound-types) - [The Tuple Type](#the-tuple-type)
  <!--toc:end-->

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

### Hello, Cargo

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

In Rust, you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is _shadowed_ by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

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

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

> Signed and unsigned refer to whether it's possible for the number to be negative--in other words, whether the number needs to have a sign with it or whether it will only ever be positive and can therefore be represented without a sign.

Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n - 1</sup> - 1 inclusive, where n is the number of bits that variant uses. So an `i8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1, so a `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.

> Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as `arch`.

You can write integer literals in any of the forms shown in the table below.

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte(u8 only)   | b'A'        |

Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`, which will have the same value as if you had specified `1000`.

**Integer Overflow**

Let’s say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error.

When you’re compiling in release mode with the `--release` flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
- Return the `None` value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.

##### Floating-Point Types

Rust also has two primitive types for floating-point numbers, which are decimal points.

- `f32` (32bits)
- `f64` (64bits)

> The default type is `f64` because on modern CPUs, it's roughly the same speed as `f32` but is capable of more precision.

> All floating-point types are signed.

```rust
fn main() {
  let x = 2.0; // f64

  let y: f32 = 3.0 // f32
}
```

> Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a single-precision float, and `f64` has double precision.

##### Numeric Operations

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

##### The Boolean Type

> Booleans are one byte in size.

```rust
fn main() {
  let t = true;

  let f: bool = false;
}
```

> The main way to use Boolean values is through conditionals, such as an `if` expression.

##### The Character Type

Rust's `char` type is the language's most primitive alphabetic type.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

> We specify `char` literals with single quotes, as opposed to string literals, which use double quotes.

> Rust's `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII, Accented letters, Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid `char` values in Rust.

#### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays

##### The Tuple Type

A `tuple` is a general way of grouping together a number of values with a variety of types into one compound type.

> Tuples have a fixed length: once declared, they cannot grow or shrink in size.

> Each position in the tuple has a type, and the types of the different values in the tuple don't have to be the same.

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1)
}
```

To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.

```rust
fn main() {
  let tup = (500, 6.4, 1);

  let (x, y, z) = tup;

  println!("The value of y is: {y}")
}
```

We can also access a tuple element directly by using a period (`.`).

```rust
fn main() {
  let x = (500, 6.1, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;
}
```

##### The Array Type

Another way to have a collection of multiple values is with an array.

> Unlike a tuple, every element of an array must have the same type.

> Unlike arrays in some other languages, arrays in Rust have a fixed length.

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
}
```

> Arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements.

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Here, `i32` is the type of each element. After the semicolon, the number `5` indicates the array contains five elements.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]
```

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

`let a = [3; 5]`

The array named `a` will contain 5 elements that will all be set to the value 3 initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a more concise way.

**Accessing Array Elements**

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

**Invalid Array Element Access**

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

This code compiles successfully. If you run this code using cargo run and enter 0, 1, 2, 3, or 4, the program will print out the corresponding value at that index in the array. If you instead enter a number past the end of the array, such as 10, you’ll see output like this:

```rust
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
