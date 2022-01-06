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

- Rust encourages you to favor immutability and why sometimes you might want to opt out.
- If one part of our code operates on the assumption that a value will never change and another part of our code changes that value, it’s possible that the first part of the code won’t do what it was designed to do. **In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change.**
- In addition to allowing this value to change, `mut` conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.（代码是写给人阅读的）
- few differences between constants and variables:
  - you aren’t allowed to use `mut` with constants.
  - You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value *must* be annotated.（猜测是，编译器需要在二进制中的 .rodata 段为 const 分配空间，所以需要指定类型）

- Rust’s naming convention for constants is to use all uppercase with underscores between words. （Go 里面更推荐使用驼峰的形式）
- Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.（与 Go 相似，Go 语言在 {} 中使用 : 符号声明的同名变量，同样不会影响到 {} 外面的值）
- Data type
  - Scalar type
    - Interger
      - i8/u8; i16/u16; i32/u32; i64/u64; i128/u128; isize/usize

    - integer literal
      - 98_222; 0xff; 0b1111_0000

    - Float-point
      - f32; f64

    - Boolean
    - Character

  - Compound types
    - tuple
      - tuple 内元素类型可以不一致
      - tuple 长度固定
      - tuple **destructuring**
      - The tuple without any values, `()`, is a special type that has only one value, also written `()`. The type is called the **unit type** and the value is called the **unit value**. **Expressions implicitly return the unit value if they don’t return any other value.**

    - array
      - 固定长度，元素类型一致

- function
  - Function bodies are made up of a series of statements optionally ending in an expression.
  - **Statements**
    - **Statements** are instructions that perform some action and do not return a value. **Expressions** evaluate to a resulting value.
    - Function definitions are also statements.
    - Statements do not return values. Therefore, you can’t assign a `let` statement to another variable. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write `x = y = 6` and have both `x` and `y`have the value `6`; that is not the case in Rust.

  - **Expressions**
    - Expressions **evaluate** to a value and make up most of the rest of the code that you’ll write in Rust. （Rust 中除了 statement 外，剩下的都是 expression）
    - Calling a function is an expression. 
    - Calling a macro is an expression. 
    - The block that we use to create new scopes, `{}`, is an expression.
    - `fn main() { let y = { let x = 3; x + 1 }; println!("The value of y is: {}", y); }`  会打印 y 的值为 4. 原因是：Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

  - In Rust, the **return value** of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.
  - Blocks of code associated with the conditions in `if` expressions are sometimes called **arms**.
  - **Blocks of code evaluate to the last expression in them**.


## 4. Understranding Ownership

ownership 解决下面三个问题：

- 解决重复 drop（free）同一片内存导致的问题 => **move** =  invalidates the copied variable + shallow copy;  move 导致了另外一个比较 tedious 的问题，该问题使用 borrow / references 解决
- 解决野指针问题（即指针指向的内容已经被释放或者被别人使用）：



- All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead. 

- ownership rules:
  - Each value in Rust has a variable that’s called its **owner**.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.

- In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.（如果没记错的话，string literal 与 const 一样放到二进制的 rodata 段里）

- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.（Rust 一定在某个时候隐式的调用了 `free()` 函数）

- When a variable goes out of scope, Rust calls a special function for us. This function is called [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), and it’s where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

- This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as a *double free* error and is one of the memory safety bugs we mentioned previously.

- Here are some of the types that implement `Copy` trait:

  - integer

  - boolean
  - float
  - character
  - tuple

- The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.（不愧是讲究一致性的语言...）
- These ampersands are *references*, and they allow you to refer to some value without taking ownership of it.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
```

- But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time.
- We *also* cannot have a mutable reference while we have an immutable one.

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
