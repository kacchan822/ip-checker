# IP Checker

[![Development Status](https://img.shields.io/badge/status-in%20development-yellow)](https://github.com/kacchan822/ip-checker)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Built with GitHub Copilot](https://img.shields.io/badge/built%20with-GitHub%20Copilot-blue)](https://github.com/features/copilot)

> ⚠️ **Work in Progress**: This project is currently under active development using GitHub Copilot for pair programming. Features may be incomplete or subject to change.

A handy utility tool for IP addresses. For example, you can check for network address overlaps or verify if an IP address matches crawler IP addresses. This tool aims to be a Swiss Army knife for IP address operations.

## Features

- **CIDR Network Overlap Detection**: Check if two CIDR ranges overlap (supports IPv4 and IPv6)
- **Crawler IP Detection**: Verify if an IP address belongs to known web crawlers
  - Built-in support for major crawlers (Googlebot, Bingbot, etc.)
  - Customizable with additional crawler sources via JSON configuration
- **Country Code Lookup**: Check country information for IP addresses (planned)

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

### Basic Commands

```bash
# Check if IP belongs to a crawler
cargo run -- crawler 8.8.8.8

# Check CIDR overlap
cargo run -- cidr 192.168.1.0/24 192.168.1.128/25

# Enable verbose output for detailed information
cargo run -- --verbose crawler 8.8.8.8
```

### Custom Crawler Sources

You can add custom crawler sources by creating an `additional_crawler_sources.json` file in the project root:

```json
[
  {
    "name": "Custom Search Bot",
    "url": "https://example.com/custom-bot-ranges.json",
    "description": "Internal custom search bot IP ranges",
    "format": "JSON"
  },
  {
    "name": "Internal Monitoring Bot",
    "url": "https://internal.example.com/monitor-ranges.txt",
    "description": "Internal monitoring service IP ranges",
    "format": "Text"
  }
]
```

**Required fields:**
- `name`: Name of the crawler bot
- `url`: URL to fetch IP ranges from  
- `description`: Description of the bot
- `format`: Data format ("JSON", "Text", etc.)

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
