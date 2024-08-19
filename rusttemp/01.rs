fn main() {
    println!("Hello World!");
}

// println! is a macro that prints text to the console.

// A binary can be generated using the Rust compiler: rustc.
//
// $ rustc hello.rs
// rustc will produce a hello binary that can be executed.
//
// $ ./hello
// Hello World!


//Regular comments which are ignored by the compiler:
// Line comments which go to the end of the line.
/*
    Block comments which go
    to the closing delimiter.
 */
// Doc comments which are parsed into HTML library documentation:
/// Generate library docs for the following item.
//! Generate library docs for the enclosing item.
