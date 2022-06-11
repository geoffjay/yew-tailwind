# Yew Tailwind Examples

## How to run

The examples are built with [trunk](https://github.com/thedodd/trunk).
You can install it with the following command:

```bash
# at some point in the future, trunk will automatically download wasm-bindgen
cargo install trunk wasm-bindgen-cli
```

Running an example only requires a single command:

```bash
# move into the directory of the example you want to run, eg. for the demo example
cd examples/demo

# build and serve the example
trunk serve --release
```

## List of examples

| Example                     | Description                                             |
| --------------------------- | ------------------------------------------------------- |
| [demo](demo)                | Demo site using components available in the library.    |
