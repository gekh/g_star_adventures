# G-Start Adventures

lang: Rust

engine: Bevy

## Run

Install Rust (_tested on rustc 1.72.0-nightly_)
https://www.rust-lang.org

Clone this repo and run

```
cargo run
```

## Release

use ./release.sh

## Optimization

```
26.9 - no opt
25.2 - opt-level = 'z'
24.3 - opt-level = 's'
26.2 - lto = "thin"
23.1 - opt-level = 'z' lto = "thin"
22.5 - opt-level = 's' lto = "thin"

wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/lesson-1.wasm
18.9

wasm-opt -O -ol 100 -s 100 -o output.wasm midota.wasm
12.4

```

## Tools

```
fast dev run:
cargo run --features bevy/dynamic_linking

create DMG image:
hdiutil create -fs HFS+ \
  -volname "Midota Game" \
  -srcfolder "Midota.app" \
  "midota.dmg"


transfer binary:
lipo "target/aarch64-apple-darwin/release/lesson-1" \
     -create -output "MyGame.app/Contents/MacOS/MyGame"
```

## Tech Stack

API framework: https://github.com/tokio-rs/axum

or https://ntex.rs + Tokio

Deployment strategy:
https://www.youtube.com/watch?v=_gMzg77Qjm0

Safe Docker:
https://docs.docker.com/network/packet-filtering-firewalls/

better docker:
https://podman.io
