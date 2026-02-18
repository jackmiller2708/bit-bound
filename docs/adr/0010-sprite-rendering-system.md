# 10. Sprite Rendering System

Date: 2026-02-18

## Status
Accepted

## Context
The runtime initially only supported pixel-level drawing primitives (`set_pixel`) and text rendering. To create visually rich game content, we need the ability to render multi-pixel sprites with transparency support. This is essential for player characters, enemies, and other game entities.

## Decision
We will implement a sprite rendering system in the `FrameBuffer`:

### Core Method
```rust
pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: &[u8], width: usize, height: usize)
```

### Key Features
- **Signed Coordinates**: Uses `i32` for x/y to support off-screen positioning (sprites can be partially visible)
- **Transparency**: Color value `0` is treated as transparent (no pixel drawn)
- **Bounds Checking**: Automatically clips sprites at screen edges, preventing out-of-bounds access
- **Variable Size**: Supports arbitrary sprite dimensions via width/height parameters
- **Flat Array Format**: Sprites stored as 1D byte arrays (row-major order)
- **Build-Time Asset Pipeline**: Sprites are authored as PNG files in `assets/`, then converted to indexed byte arrays by `build.rs` using the `image` crate. The generated code is written to `src/sprites.rs`.

### Implementation Details
- Nested loops iterate over sprite dimensions
- Skip transparent pixels (color 0)
- Bounds check each pixel before drawing
- Uses existing `set_pixel()` for actual rendering

## Consequences

### Positive
- **Flexible Positioning**: Signed coordinates allow sprites to smoothly enter/exit screen
- **Safe**: Bounds checking prevents crashes from off-screen sprites
- **Transparent Overlays**: Color 0 transparency enables layered sprite composition
- **No Allocation**: Sprites are compile-time constants, no heap usage
- **Simple Format**: Flat byte arrays are easy to inspect and debug
- **Artist-Friendly Workflow**: Sprites are authored as standard PNG files using the 4-color palette, then automatically converted at build time. No manual byte array editing required.

### Negative
- **No Optimization**: Draws every non-transparent pixel individually (no batching)
- **No Rotation/Scaling**: Only supports 1:1 pixel rendering
- **Build Dependency**: The `image` crate is required as a build dependency for PNG decoding
- **Strict Palette Enforcement**: PNGs must use exact RGBA values matching the 4-color palette; unexpected colors cause a build panic
- **Limited Palette**: Constrained to 4 colors (0-3)

## Alternatives Considered
- **Tile-based rendering**: Too rigid for arbitrary sprite sizes
- **Blit operations**: More complex, harder to debug
- **Pre-transformed sprites**: Would require more memory for rotations/flips
