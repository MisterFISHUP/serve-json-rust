# serve-json-rust

Serving json files by Rust using [`axum`](https://github.com/tokio-rs/axum).

## Run

```
cargo run --bin serve-json-rust
```

## Build

Build without specifying a target:

```bash
cargo build --release
```

Build for ARM64 macOS using docker (cf [rust-linux-darwin-builder](https://github.com/joseluisq/rust-linux-darwin-builder))

```bash
docker run --rm \
    --volume "${PWD}":/root/src \
    --workdir /root/src \
      joseluisq/rust-linux-darwin-builder:1.68.0 \
        sh -c "cargo build --release --target aarch64-apple-darwin"
```

> Some prebuilt binaries can be found in the `binaries` folder.

## Usage

1. Place json files in the `data` folder
1. Run the `serve-json-rust` executable (or the binaries in the `binaries` folder)
1. Now the json files in `data` are served: visit `/path` on port 8000 => `path.json`, or `path/index.json`, or 404

> Note: This is my first `axum` program 😊.
