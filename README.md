# Ray-Tracing Voxel Renderer

Project for COP4520 Spring 2025.

## Contributing

See `CONTRIBUTING.md`

## Building

MSRV (Minimum Supported Rust Version): 1.84

Run `cargo build` for a debug build or `cargo build --release` for a release build.

To run the voxel renderer, run `cargo run` or `cargo run --release`.

## Tracing

To track traces, you can use [Tracy v0.11.1](https://github.com/wolfpld/tracy).

Run the Tracy client and have it wait for a connection.

Then run `cargo run --release --features trace` to start collecting traces.

## Output

![a simple scene](./render.png)
