# Stacker

Little game for the [Bevy Jam #2](https://itch.io/jam/bevy-jam-2)

## Game

Stack the highest amount of pieces in the board.
If 3 pieces falls outside, the game ends.
You can combine pieces of the same color.

## Commands

- Drag and drop pieces with your mouse
- Stick 2 pieces of the same color together to combine them

## Build

1. Compile wasm app

```sh
cargo build --release --target wasm32-unknown-unknown
```

2. Create JS bindings

```sh
wasm-bindgen --out-dir ./dist/target --target web ./target/wasm32-unknown-unknown/release/stacker.wasm
```

## Publish

- Start a local web server

```sh
basic-http-server dist
```

- Publish to itch.io

```sh
zip dist.zip dist/**/*
```
