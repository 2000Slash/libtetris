# Examples

This lib has three examples right now: Rust, native and wasm. 
To run the rust one: `cargo run --example main`.

To run the wasm one, install wasm-pack and then run: `wasm-pack build --target web --features wasm` to build. Copy the resulting pkg folder to examples/wasm and open the index.html.

To compile for native, include the feature native `cargo build --features native`. The main.c and test.c show how to use it.