# Dipole Simulation

Started from [this project](https://github.com/chinedufn/webgl-water-tutorial).

```sh
# You can use any static file server that properly sets the
# `application/wasm` mime type
cargo install https

git clone https://github.com/chinedufn/dipole-simulation
cd dipole-simulation

# A version of Rust that can compile wasm-bindgen-cli version 0.2.29
cargo install -f wasm-bindgen-cli --version 0.2.29 # Or download a release binary

# Build
./build.sh

## Opens your browser to http://localhost:8000  where the demo will be running
http -m wasm:application/wasm
```
