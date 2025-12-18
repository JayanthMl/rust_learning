# Chapter 01 Getting Started

>  **The Rust Programming Language(*https://doc.rust-lang.org/book/ch01-03-hello-cargo.html*)**

**1.3 Hello Cargo.**

In this section we see Cargo which is rust's build system and package manager.

Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

### Some useful Cargo commands
$ cargo new <project_name> -> creates a new project

$ cargo build -> builds the project and creates an executable in the /target/debug/file_name Note: for windows we have file_name.exe

$ cargo run -> builds and runs the project

$ cargo check -> compiles the code but doesn't create an executable helps check for compilation errors this comes handy as it's faster than the *cargo build* as it does not create an executable

$ cargo build --release -> used to build for production 

> Note: By default the cargo build would make a folder target/debug which is mainly for development for production builds we should be using the *--release* flag this creates a release folder in the target indicating that it's the production build also we use the same for benchmarking as this is faster but it takes more time than the normal *cargo build* which builds faster