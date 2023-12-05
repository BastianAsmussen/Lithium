# About

This is a simple programming language written in Rust for learning purposes.
It is a statically typed, compiled language with a syntax very similar to Rust.

## Statistics
[![Build and Test](https://github.com/BastianAsmussen/Lithium/actions/workflows/build.yml/badge.svg)](https://github.com/BastianAsmussen/Lithium/actions/workflows/build.yml)  
![Code Size](https://img.shields.io/github/languages/code-size/BastianAsmussen/Lithium)  
![License](https://img.shields.io/github/license/BastianAsmussen/Lithium)  

## Table of Contents

- [About](#about)
- [Compiling](#compiling)
- [Installing](#installing)
- [Testing](#testing)
- [Benchmarking](#benchmarking)
- [Usage](#usage)
- [Syntax](#syntax)
- [Contributing](#contributing)

## Compiling

To compile the Lithium compiler, we use Cargo (get it [here](https://rustup.rs/)).

```bash
$ cargo build --release --workspace
```

## Installing

To install the compiler, use the following command:

```bash
$ cargo install --path .
```

## Testing

To run the tests, use the following command:

```bash
$ cargo test --workspace
```

## Benchmarking

To run the benchmarks, use the following command:

```bash
$ cargo bench --workspace
```

## Usage

To compile a Lithium program, use the following command:

```bash
$ lithium --file </path/to/file.lt>
```

## Syntax

The syntax is very simple. Here is an example program:

```lt
// This is a comment.
fn greet(name: str) -> str {
    return "Hello, " + name + "!";
}

print(greet("World"));
```

## Contributing

If you want to contribute to Lithium, please read the [CONTRIBUTING](CONTRIBUTING.md) file for more information.
