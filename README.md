# Rust - Complete Course

## Course Projects

- [minigrep CLI tool](https://github.com/AntonioMartinezFernandez/udemy-rust-curso-completo/tree/main/minigrep)
- [Tokio TCP Chat Server](https://github.com/AntonioMartinezFernandez/udemy-rust-curso-completo/tree/main/tokio-chat-server)
- [Web Server](https://github.com/AntonioMartinezFernandez/udemy-rust-curso-completo/tree/main/web-server)

## Links

- [Udemy Course](https://www.udemy.com/course/curso-completo-rust/)
- [Rust Installation](https://www.rust-lang.org/tools/install)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings -Rust exercises-](https://github.com/rust-lang/rustlings)

## First steps

Create project (create new folder with a Rust project inside)

```
cargo new --edition 2021 project-name
```

Init project (init a Rust project in a folder previously created)

```
cargo init
```

Create library

```
cargo new library-name --lib
```

Check code

```
cargo check
```

Check code with clippy

```
cargo clippy
```

Fix linting suggestions automatically with clippy

```
cargo clippy --fix
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

## Linting

- [Lint levels](https://doc.rust-lang.org/rustc/lints/levels.html)
- [Clippy](https://doc.rust-lang.org/nightly/clippy/)

## Resources

- [Rust github](https://github.com/rust-lang)
- [Official repository](https://github.com/rust-lang/rust)
- [Standard library reference](https://doc.rust-lang.org/std/index.html)
- [Compiler Error index](https://doc.rust-lang.org/error-index.html)
- [Rust Blog](https://blog.rust-lang.org/)
- [Inside Rust Blog](https://blog.rust-lang.org/inside-rust/)
- [Rust YouTube channel](https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA)
- [Rust podcast](https://rustacean-station.org/)
- [Reddit Rust community](https://www.reddit.com/r/rust/)
- [Slack Rust community](https://rust-slack.herokuapp.com/)
- [CLI Rust applications](https://rust-cli.github.io/book/index.html)
- [Embedded Rust applications](https://doc.rust-lang.org/stable/embedded-book/)
- [Rust with STM32 microcontrollers YouTube course](https://www.youtube.com/watch?v=o_alVYMBBco&list=PLL2SCPK5xSRWBPj-nKOVYIhxRw7C4kYeI)
- [WebAssembly Rust applications](https://rustwasm.github.io/docs/book/)
- [Awesome Rust projects](https://github.com/rust-unofficial/awesome-rust)
- [Rust repository trends](https://github.com/trending/rust)
- [Tokio Async Runtime](https://tokio.rs/tokio/tutorial)
- [Tokio vs. Async-std](https://medium.com/@AlexanderObregon/async-programming-in-rust-exploring-tokio-and-async-std-97d4b524cef0)

## Books

- [Programming Rust: Fast, Safe Systems Development](https://www.amazon.com/-/es/Jim-Blandy-ebook/dp/B0979PWD4Z/)
- [Rust for Rustaceans](https://www.amazon.com/-/es/Jon-Gjengset-dp-1718501854/dp/1718501854/)
- [Rust in Action](https://www.amazon.com/-/es/Tim-McNamara-ebook-dp-B098BNGMWH/dp/B098BNGMWH)
