# About

This is a simple programming language written in Rust for learning purposes.
It is a statically typed, compiled language with a syntax very similar to Rust.

## Statistics
[![Build](https://github.com/BastianAsmussen/Lithium/actions/workflows/build.yml/badge.svg)](https://github.com/BastianAsmussen/Lithium/actions/workflows/build.yml)  
[![Tests](https://github.com/BastianAsmussen/Lithium/actions/workflows/test.yml/badge.svg)](https://github.com/BastianAsmussen/Lithium/actions/workflows/test.yml)  
[![Benchmarks](https://github.com/BastianAsmussen/Lithium/actions/workflows/bench.yml/badge.svg)](https://github.com/BastianAsmussen/Lithium/actions/workflows/bench.yml)  
![License](https://img.shields.io/github/license/BastianAsmussen/Lithium)

## Table of Contents

- [About](#about)
- [Compiling](#compiling)
- [Testing](#testing)
- [Benchmarking](#benchmarking)
- [Usage](#usage)
- [Syntax](#syntax)
- [Contributing](#contributing)

## Compiling

To compile the compiler, we use Cargo (get it [here](https://rustup.rs/)).

```bash
$ cargo build --release --workspace
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
$ lithium <input file> <output file>
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

## License

Lithium is licensed under the MIT license. See the [LICENSE](LICENSE) file for more information.

## Contributing

If you want to contribute to Lithium, please read the [CONTRIBUTING](CONTRIBUTING.md) file for more information.
