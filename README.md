# IP Checker

A handy utility tool for IP addresses. For example, you can check for network address overlaps or verify if an IP address matches crawler IP addresses. This tool aims to be a Swiss Army knife for IP address operations.

## Features

- Network address overlap detection
- Crawler IP address matching (Googlebot, etc.)

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Build

```bash
git clone https://github.com/kacchan822/ip-checker.git
cd ip-checker
cargo build --release
```

## Usage

### Help

```bash
cargo run -- --help
```

## Development

### Run Tests

```bash
cargo test
```

### Run Linter

```bash
cargo clippy
```

### Format Code

```bash
cargo fmt
```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on how to get started.

## Author

- [@kacchan822](https://github.com/kacchan822)
