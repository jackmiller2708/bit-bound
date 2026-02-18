# 11. Binary Asset Pipeline

Date: 2026-02-18

## Status
Accepted

## Context
Initially, the asset pipeline was tightly coupled with the engine's build process via `build.rs`. The build script used the `image` crate to parse PNG files and generate Rust source code (`src/sprites.rs`) containing inline byte arrays. This had several drawbacks:
- **Build Times**: Adding `image` as a build dependency significantly increased compilation times for the core project.
- **Engine Purity**: The engine was forced to depend on the asset pipeline logic and source art locations.
- **Portability**: Generating source code is less flexible than consuming binary artifacts.
- **Separation of Concerns**: The engine should not know about PNGs, palettes, or conversion logic.

## Decision
We will decouple the asset pipeline from the engine and shift to a purely binary data flow:

1.  **Standalone CLI Tool**: A new tool `spritec` (sprite compiler) is introduced in `tools/spritec/`. It is a dedicated workspace member responsible for all asset conversion logic.
2.  **Explicit Documentation**: The binary format used by the engine is documented in `docs/sprite_format.md`.
3.  **Binary Artifacts**: `spritec` reads source art from `assets/raw/` and outputs binary `.2bpp` files to `assets/processed/`.
4.  **Engine Consumption**: The engine (and game layer) consumes these binary files using `include_bytes!()`. The engine remains completely data-agnostic, with no dependencies on image parsing crates.
5.  **Agnostic Build**: Root `build.rs` is removed. Asset conversion becomes an explicit or manual step (`cargo run -p spritec`), rather than an automatic part of every `cargo build`.

## Consequences

### Positive
- **Fast Build Times**: The main crate no longer compiles the `image` crate during build.
- **Strict Separation**: Improved architectural boundaries. The engine and conversion tools can evolve independently.
- **Format Stability**: Forcing the engine to consume a fixed binary format ensures the data contract is well-defined and documented.
- **Reproducibility**: Binary assets are treated as stable artifacts rather than floating source-generated code.

### Negative
- **Manual Step**: Developers must remember to run `spritec` when art changes (though this could be automated via external scripts or CI).
- **Toolchain Complexity**: Introducing multiple workspace members increases the structural surface area of the repository.

## Alternatives Considered
- **Keep `build.rs` but output binary**: Improved build times slightly, but still coupled engine build to art processing logic.
- **Runtime decoding**: Rejected because the runtime environment is strictly constrained and does not support heap allocation or complex file parsing.
