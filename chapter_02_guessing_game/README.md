# Chapter 02 Programming a Guessing Game

>  **The Rust Programming Language**
>  ***(https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)***

In this section, we build a simple CLI game where the user guesses a number. Along the way we'll learn key Rust concepts like `let`, `match`, methods, associated functions, and how to work with external crates.

### Commenting in Rust
In Rust, we use `//` to comment the code.
**Example:**    
```rust
// This is a comment
```

### Adding Crates (External Libraries)
- Add the library to our *Cargo.toml* file
```toml
[dependencies]
<name> = "<version>"
```

- Using Cargo
```bash
cargo add <name_of_the_crate>
```

**Note: Find the libraries in *crates.io***

> Rust uses Semantic Versioning (https://semver.org/)
