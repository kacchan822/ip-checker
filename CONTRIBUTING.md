# Contributing to IP Checker

Thank you for your interest in contributing to IP Checker! We welcome contributions from everyone.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo
- Git

### Setting up the Development Environment

1. Fork this repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ip-checker.git
   cd ip-checker
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Development Workflow

### Making Changes

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bugfix-name
   ```

2. Make your changes
3. Ensure your code follows the project's style:
   ```bash
   cargo fmt
   ```
4. Run the linter:
   ```bash
   cargo clippy
   ```
5. Run tests:
   ```bash
   cargo test
   ```

### Submitting Changes

1. Push your changes to your fork:
   ```bash
   git push origin your-branch-name
   ```
2. Create a pull request from your fork to the main repository
3. Describe your changes clearly in the pull request description
4. Link any relevant issues

## Code Style

- Follow standard Rust formatting (use `cargo fmt`)
- Follow Rust naming conventions
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions focused and small

## Testing

- Write tests for new functionality
- Ensure all existing tests pass
- Include both unit tests and integration tests when appropriate
- Test edge cases

## Reporting Issues

When reporting issues, please include:

- A clear description of the problem
- Steps to reproduce the issue
- Expected vs actual behavior
- Your environment (OS, Rust version, etc.)
- Relevant error messages or logs

## Questions?

If you have questions about contributing, feel free to:

- Open an issue with the "question" label
- Reach out to the maintainers

Thank you for contributing!
