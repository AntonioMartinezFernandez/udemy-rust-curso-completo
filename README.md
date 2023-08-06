# Rust - Complete Course

## Links

- [Udemy Course](https://www.udemy.com/course/curso-completo-rust/)
- [Rust Installation](https://www.rust-lang.org/tools/install)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings -Rust exercises-](https://github.com/rust-lang/rustlings)

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

- [Variables and Constants](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
- [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Statement vs Expression](https://nickymeuleman.netlify.app/garden/rust-expression-statement)
- [Ownership & Borrowing](https://progressivecoder.com/understanding-rust-ownership-and-borrowing-with-examples/)
