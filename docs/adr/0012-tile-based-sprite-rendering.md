# 12. Tile-Based Sprite Rendering

Date: 2026-02-18

## Status
Accepted (Supersedes [0010 — Sprite Rendering System](0010-sprite-rendering-system.md))

## Context
The previous sprite system (ADR 0010) used flat byte arrays with 1 byte per pixel. While easy to coordinate, this was inefficient:
- **Redundant Data**: Each 2-bit color index occupied 8 bits (1 byte).
- **Incompatible with Legacy Patterns**: Real hardware (GameBoy, NES) used planar bit-packing and tile-based systems for significant memory savings and hardware optimization.
- **Unstructured Layout**: Sprites were arbitrary rectangles, which made hardware-like optimization (like tile caching or shared patterns) difficult.

## Decision
We will move to a tile-aligned, GameBoy-style 2bpp planar rendering system:

1.  **Tile Alignment**: All sprites must have dimensions that are multiples of 8. Sprites are padded to tile bounds during the conversion process.
2.  **2bpp Planar Encoding**: Pixels are encoded using 2 bits per pixel, stored in a "planar" fashion (low-plane and high-plane bytes per row). This results in exactly 16 bytes per 8×8 tile.
3.  **Render Primitive**: The engine now implements `draw_tile(x, y, data: &[u8])`, which is responsible for decoding a single 16-byte tile and writing it to the framebuffer.
4.  **Composite Sprite Rendering**: `draw_sprite` is refactored to iterate over a tile grid, delegating the rendering of each individual 8×8 block to `draw_tile`.
5.  **Sprite Metadata**: The `Sprite` struct is expanded to include `tiles_x` and `tiles_y`, allowing the engine to be completely agnostic of the raw pixel dimensions during the rendering loop.

## Consequences

### Positive
- **50% Memory Reduction**: Assets on disk and in memory (if loaded dynamically) take half the space of the old format (2 bits vs 8 bits per pixel).
- **Hardware Realism**: The system now accurately emulates the data structures found on actual 8-bit game consoles.
- **Improved Performance potential**: Processing a tile at a time allows for easier optimization and potential future hardware-assisted rendering logic.
- **Implicit Transparency**: The encoding naturally preserves the "0 is transparent" convention.

### Negative
- **Encoding Complexity**: The planar format is mentally harder to visualize than a flat byte array, requiring robust conversion tools and documentation.
- **Padding Overhead**: Sprites that are slightly off-size (e.g., 35x16) must be padded to the nearest 8-pixel boundary (e.g., 40x16), slightly increasing data size for odd-shaped assets.

## Alternatives Considered
- **Flat 2-bit packing**: Packing 4 pixels into one byte (0-1, 2-3, 4-5, 6-7). This is more intuitive but was rejected to match the GameBoy-style planar bitplanes which allowed certain hardware effects more easily.
- **Compressed assets**: Runtime decompression was rejected due to memory and CPU constraints.
