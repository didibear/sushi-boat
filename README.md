# Sushi boat

![image](https://img.itch.zone/aW1nLzk5MTE3OTIucG5n/original/XlNCfD.png)

Little game for the [Bevy Jam #2](https://itch.io/jam/bevy-jam-2)

## Game

Combine the given items to create the sushi boat !

## Commands

- Drag and drop items with your mouse
- Stick two items together to combine them

## Build

1. Compile wasm app

```sh
cargo build --release --target wasm32-unknown-unknown
```

2. Create JS bindings

```sh
wasm-bindgen --out-dir ./dist/target --target web ./target/wasm32-unknown-unknown/release/stacker.wasm
```

3. Copy assets to the `dist` folder
```sh
cp -r assets dist
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
