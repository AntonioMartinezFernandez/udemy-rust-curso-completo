# YouTube Search Project

Fullstack WASM application doing use of [yew](https://crates.io/crates/yew), a Rust framework for creating multi-threaded front-end web apps with WebAssembly (basically an alternative to [React](https://react.dev/) for Rust).

## Run Project

You will need a YouTube API KEY. [Create one](https://console.cloud.google.com/apis/credentials) and put it on the 'env.rs' file.

Yew need to have installed WebAssembly target and [Trunk](https://crates.io/crates/trunk)

```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Start WASM application with Trunk

```
trunk serve
```
