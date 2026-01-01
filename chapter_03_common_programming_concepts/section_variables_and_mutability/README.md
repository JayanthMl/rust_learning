# Chapter 03 Common Programming Concepts

## 3.1 Variables and Mutability

>  **The Rust Programming Language**
>  ***(https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)***

In this section, we'll explore how variables and mutability work in Rust.


### Variables
We define a variable in Rust using the `let` keyword:

```rust
let x = 6;
```

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
> In the above example we can see that when we assign `x` to `6` this will result in a compile-time error because variables are by default immutable in Rust. We can use the `mut` keyword before the variable name to make it mutable.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
> Here we can see the use of the `mut` keyword which allows us to assign a new value to  `x`. In Rust, mutability only applies within the scope of the variable, so outside of this scope, the variable is immutable again.

### Constants
Rust has constants, which hold fixed values throughout the program's execution. Unlike variables, which can be made mutable using `mut` keyword, ***constants cannot be changed***. We declare constants using the `const` keyword, and we must explicitly annotate their type.
Constants can be declared in any scope, which makes them useful for values that need to be known globally throughout the code. Additionally, constants can only be set to a constant expression, not values computed at runtime.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
The above is an example of declaring a constant in Rust.

### Shadowing
We can declare a new variable with the same name as a previous variable this is known as shadowing. This can be useful for reusing variable names with new values. Here's an example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

> Output:
> The value of x in the inner scope is: 12
> The value of x is: 6

The above is an example of shadowing in Rust.