# Rust - Complete Course

## Links

- [Udemy Course](https://www.google.com/url?q=https://meet.google.com/mpd-mxug-rqv?hs%3D224&sa=D&source=calendar&usd=2&usg=AOvVaw1urQxtnbJ6EdBGUJ88mMEI)
- [Rust Installation](https://www.rust-lang.org/tools/install)
- [Rust Book](https://doc.rust-lang.org/book/)

## First steps

Create project

```
cargo new --edition 2021 project-name
```

Create library

```
cargo new library-name --lib
```

Check code

```
cargo check
```

Compile

```
rustc ./main.rs
```

Build

```
cargo build
```

Build and optimize

```
cargo build --release
```

Execute

```
./main
```

Build + Execute

```
cargo run
```

### Install dependencies (manually)

1. Search the library in the [crate registry - crates.io](https://crates.io/)
2. Include the dependency in the `cargo.toml` file
3. Import the dependency in the code (`use library::method;`)
4. Run `cargo check`

### Install dependencies (automatically)

Search the library in the [crate registry - crates.io](https://crates.io/) and execute:

```
cargo add library-name

```

Examples:

```
cargo add rand
cargo add serde --features derive
```

## Tips

### VS code extensions

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Rust tools
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) - TOML support
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - Cargo.toml check
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - Debugger
- [C/C++ Extension Pack](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools-extension-pack) - C/C++ tools
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - Errors description in-line

### Cargo Watch (hot reloading)

Install

```
cargo install cargo-watch
```

Run

```
cargo watch -x run
```

## Concepts
