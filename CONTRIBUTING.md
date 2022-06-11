# Contribution Guide

## Setting up your local development environment

### Add the Wasm target

```bash
rustup target add wasm32-unknown-unknown
```

### Install [cargo-make](https://github.com/sagiegurari/cargo-make)

```bash
cargo install cargo-make
```

You can use the following command to list all available tasks for Yew:

```bash
cargo make --list-all-steps
```

The most important tasks are outlined below.

## Tests

To run all tests, use the following command:

```bash
cargo make tests
```

## Linting

The following command checks the code using Rustfmt and Clippy:

```bash
cargo make lint
```

To automatically fix formatting issues, run `cargo fmt` first.
