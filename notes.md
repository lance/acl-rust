# Rust

Systems programming language, e.g. C, C++, Objective C, etc.
"Curly Brace" Syntax like C, Javascript, etc.
Version 0.13.0 - nightly build

## Focus

Saftey + Speed

### Safety

No null pointers?

### Speed

Native, compiled language

## Why use rust?

Memory safety without garbage collection. Memory safety is enforced
primarily at compile time. This way, you do not incur runtime overhead
as found in memory-managed/interpreted languages, but rather can use
systems level language features that often result in bugs confidently.

## Data Types

  * Strings
  * Number (int)
  * Tuples (fixed-size set with type inference and destructuring)

    let (x, y, z) = (1i, 2i, 3i);
    println!("x is {}", x);


## Tools

Cargo is the package manager.

Create a new project

    $ cargo new hello-world --bin

### What did it do?

    $ cd hello-world
    $ ls *
    Cargo.toml
    
    src:
    main.rs
    $ cat Cargo.toml
    [package]
    name = "hello-world"
    version = "0.0.1"
    authors = ["Lance Ball <lball@redhat.com>"]


## Dependencies

    [dependencies.semver]
    git = "https://github.com/rust-lang/semver.git"

## References

Rust Language - http://rust-lang.org
Cargo Package Manager - http://doc.crates.io
30 Minute Intro - http://doc.rust-lang.org/nightly/intro.html
Playground - http://play.rust-lang.org/
