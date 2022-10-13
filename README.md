# WASM4NN

WASM4NN is an experimental project to run ML inference with WebAssembly(WASM).

## Flow

## Prerequisites

WASM4NN builds with **EfficientFormer**, **Rust**, **tract** and **WebAssembly**.
See also
- [EfficientFormer: Vision Transformers at MobileNet Speed @ NIPS2022](https://github.com/snap-research/EfficientFormer)
- [Rust](https://www.rust-lang.org/)
- [tract:Tiny, no-nonsense, self-contained, Tensorflow and ONNX inference](https://github.com/sonos/tract)
- [WebAssembly](https://webassembly.org/)

## Getting Started

1. Clone WASM4NN repo

    ```
    git clone https://github.com/markwu010/WASM4NN
    ```

1.  Switch to the curated example path: **onnx-efficientTransformer**

    ```
    cd examples/onnx-efficientTransformer
    ```

1. Build the example to the WASM target

   ```
   cargo build --release --target wasm32-wasi
   ```
   Note: Add the target **wasm32-wasi** in the rust toolchain.
   ```
   rustup target add wasm32-wasi
   ```
   See more: [Managing the wasm32-wasi target](https://bytecodealliance.github.io/cargo-wasi/steps.html#managing-the-wasm32-wasi-target)

1. Check the result in target folder
   ```
   cd ../../target/wasm32-wasi/release 
   ```
   The expected output is a wasm file. 
   In this example: **example-onnx-efficientTransformer.wasm** is the file we want.

###  Run with WebAssembly runtime

WebAsembly needs a WebAssembly runtime to execute the logic inside.
Here we recoomend [Wasmtime](https://wasmtime.dev/) for the WebAssembly module execution.

You could run the AI inference with the following command once WebAssembly runtime is ready.
```
USAGE:
    wasmtime --dir=. ./example-onnx-efficientTransformer.wasm -- --image-path <IMAGE_PATH> --model-path <MODEL_PATH>
```

Build once, run everywhere!

## License

MIT.
