use wasm_bindgen::prelude::*;

// FFI: Foreign Function Interface
// ABI: Application Binary Interface
// 'extern' keyword, with ABI type 'C' in a block code, allow us to execute external functions at low level
#[wasm_bindgen]
extern "C" {
    // This function ('alert') will be declared in an external language (Javascript)
    pub fn alert(s: &str);
}

// Communicate from Rust to Javascript with 'wasm_bindgen' macro
#[wasm_bindgen]
pub fn say_hi_from_rust() {
    /*****************************************************************************
     *
     *        WEBASSEMBLY
     *
     *  1. Install wasm-pack tool
     *  2. Add 'crate-type = ["cdylib", "rlib"]' to lib section in Cargo.toml file
     *  2. Add 'wasm-bindgen = "0.2.87"' to dependencies section in Cargo.toml file
     *  3. Run from terminal: wasm-pack build
     *
     ****************************************************************************/

    alert("Hi from Rust WebAssembly App!");
}
