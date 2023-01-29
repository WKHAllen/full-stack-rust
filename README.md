# Full Stack Rust

A template webview desktop application made with Rust, using Yew for frontend and Tauri for backend.

## Requirements

The only required tools besides `cargo` and `make` are [`trunk`](https://trunkrs.dev/) and the [`tauri CLI`](https://tauri.app/):

```sh
# Install trunk
$ cargo install --locked trunk

# Install the tauri CLI
$ cargo install tauri-cli

# Add the WASM target
$ rustup target add wasm32-unknown-unknown
```

## Build and run

Make is used to streamline the development process:

```sh
# Run in development mode, watching for file changes
$ make run

# Build for distribution
$ make build
```
