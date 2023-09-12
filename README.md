# Lithium

A simple programming language.

## Compiling

To compile the compiler, we use Cargo (get it [here](https://rustup.rs/)).

```bash
$ cargo build --release
```

## Testing

To run the tests, use the following command:

```bash
$ cargo test
```

## Docker Image

To build the Docker image, use the following command:

```bash
$ docker build -t lithium .
```

To run the Docker image, use the following command:

```bash
$ docker run -it --rm lithium
```

## Usage

To compile a Lithium program, use the following command:

```bash
$ lt <input file> <output file>
```

## Syntax

The syntax is very simple. Here is an example program:

```lt
// This is a comment.
fn hello(name: str) {
    print("Hello, " + name + "!");
}

hello("world");
```

## License

Lithium is licensed under the MIT license. See the [LICENSE](LICENSE) file for more information.

## Contributing

If you want to contribute to Lithium, please read the [CONTRIBUTING](CONTRIBUTING.md) file for more information.
