# THE BOOK NOTING

## 1. Getting Started

- Rust is an **ahead-of-time compiled** language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
- Cargo is Rust's build system and package manager.
- Creating a project with Cargo.
- In Rust, packages of code are referred to as **crates**.（类似 Go 中的 module）
- Cargo expects your source files to live inside the **src** directory.
- `cargo check` quickly checks your code to make sure it compiles but doesn't produce an executable.
- `cargo build --release`

## 2. Programming a Guessing Game

- `let` statement is used to create a **variable**. In Rust, variables are **immutable** by default.
- `let mut guess = String::new()` : `new` is an **associated function** of the `String` type.
- The `&` indicates that this argument is a **reference**, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 
- The `Result` types are [*enumerations*](https://doc.rust-lang.org/book/ch06-00-enums.html), often referred to as **enums**. An enumeration is a type that can have a fixed set of values, and those values are called the enum’s **variants**. For `Result`, the variants are `Ok` or `Err`.
- Remember that a **crate** is a collection of Rust source code files. The project we’ve been building is a **binary crate**, which is an executable. The `rand` crate is a **library crate**, which contains code intended to be used in other programs.
- The number `0.8.3` is actually shorthand for `^0.8.3`, which means any version that is at least `0.8.3` but below `0.9.0`. Cargo considers these versions to have public APIs compatible with version `0.8.3`, and this specification ensures you'll get the latest patch release that will still compile with the code in this chapter.
- [Ensuring Reproducible Builds with the *Cargo.lock* File](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#ensuring-reproducible-builds-with-the-cargolock-file): Cargo.lock 中记录了上一次 `cargo build` 时使用的版本
- A `match` expression is made up of **arms**. An arm consists of a **pattern** and the code that should be run if the value given to the beginning of the `match` expression fits that arm’s pattern. Rust takes the value given to `match` and looks through each arm’s pattern in turn.
- **shadow**

## 3. Common Programming Concepts

## 4. Understranding Ownership

## 5. Using Structs to Structure Related Data

## 6. Enums and Pattern Matching

## 7. Managing Growing Projects with Packages, Crates, and Modules

## 8. Common Collections

## 9. Error Handling

## 10. Generic Types, Traits, and Lifetimes

## 11. Writing Automated Tests

## 12. An I/O Project: Building a Command Line Program

## 13. Functional Language Features: Iterators and Closures

## 14. More about Cargo and Crates.io

## 15. Smart Pointers

## 16. Fearless Concurrency

## 17. Object Oriented Programming Features of Rust

## 18. Patterns and Matching

## 19. Advanced Features

## 20. Final Project: Building a Multithreaded Web Server
