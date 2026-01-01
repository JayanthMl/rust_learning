# Chapter 03 Common Programming Concepts

## 3.2 Data Types

>  **The Rust Programming Language**
>  ***(https://doc.rust-lang.org/book/ch03-02-data-types.html)***

In this section, we'll explore how data types work in Rust.

### Data Types
Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so that it knows how to work with that data. We’ll look at two data type subsets: **scalar and compound**.

Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as when we converted a String to a numeric type using parse in the “Comparing the Guess to the Secret Number” section in Chapter 2, we must add a type annotation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

### Scalar Types
Scalar types represents a single value. Rust has four primary scalar types:
- Integers
- Floating Point Numbers
- Booleans
- Characters

### Integer Types
Integers are numbers without a fractional component, we can look the table below showing the length and type(signed/unsigned) of the Integer.

###### Table 3‑1: Integer Types in Rust

| Length                  | Signed  | Unsigned |
|------------------------|--------|----------|
| 8‑bit                   | `i8`   | `u8`     |
| 16‑bit                  | `i16`  | `u16`    |
| 32‑bit                  | `i32`  | `u32`    |
| 64‑bit                  | `i64`  | `u64`    |
| 128‑bit                 | `i128` | `u128`   |
| Architecture‑dependent  | `isize` | `usize` |

Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where n is the number of bits that variant uses. So, an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127. 
Unsigned variants can store numbers from 0 to 2n − 1, so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.

Rust defaults to i32 when not explicitly told about the integer type

### Floating-Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

Here's an example showcasing floating-point numbers:
```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### The Boolean Type
As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool. For example:
```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```
The main way to use Boolean values is through conditionals, such as an if expression.

### The Character Type
Rust’s char type is the language’s most primitive alphabetic type. Here are some examples of declaring char values:
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```
>**Note:** We specify char literals with single quotation marks, as opposed to string literals, which use double quotation marks. Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust. Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.

### Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### The Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: Once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

