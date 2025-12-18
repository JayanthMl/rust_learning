# Chapter 01 Getting Started

>  **The Rust Programming Language**
>  ***(https://doc.rust-lang.org/book/ch01-01-installation.html & https://doc.rust-lang.org/book/ch01-02-hello-world.html)***

**1.1 Installation**, **1.2 Namaskaraa Guruu!(Hello World)** 
> $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh (For Linux/macOS)

> *https://rust-lang.org/tools/install/* (For Windows)

You will also need a linker, if you get linker errors install a C compiler which usually has a linker present, on most Linux machines we have a C compiler, for Windows you can run:
> $ rustup toolchain install stable-x86_64-pc-windows-gnu
> $ rustup default stable-x86_64-pc-windows-gnu

For macOS you can run:
> $ xcode-select --install

Check if rust has been installed by running

> $ rustc --version 

Now let's get started with our first program Namaskaraa Guruu!!
> $ mkdir hello_world
> $ cd hello_world

Rust extension for files is .rs and the convention is to have a main.rs file which is like the entry point you can choose to name your file anything but the convention is to use main.rs

The entry point in Rust is always the main function in rust

We can use rustfmt to format Rust code:
> $ rustfmt main.rs

If rustfmt is not installed:
> $ rustup component add rustfmt


### Compilation and Execution

We use the rustc <file_name> to compile the rust code which creates a binary executable
> $ rustc main.rs

The above command would create a binary executable named main(main.exe in Windows)

We can use ./main to run the executable for Windows it'll be .\main.exe



>Note:
>
>To update rust run
>> $ rustup update
>
>To uninstall run
>> $ rustup self uninstall
>
>You can run to open the documentation locally
>> $ rustup doc