# bit-bound

A deliberately constrained game runtime designed to emulate the discipline of early game development environments.

## Vision

This project is a constrained game runtime that enforces hard limits and explicit resource management. The goal is architectural clarity through constraint, rather than modern engine abstractions.

## Core Philosophy

- **Emulate old hardware limits**: 160 × 144 resolution, 4-color palette.
- **Enforce strict memory ceilings**: 1 MB fixed total memory pool.
- **Fixed-capacity systems**: No dynamic allocation (Vec, Box, String).
- **Deterministic 60 FPS fixed timestep**: Predictable performance and behavior.
- **Explicit resource usage**: All resources are pre-allocated and measurable.
- **Punish sloppy architecture**: Hard failures when constraints are violated.

## Technical Constraints

- **Resolution**: 160 × 144 (2 bits per pixel)
- **Memory**: 1 MB (Fixed Arena System)
- **Timestep**: 60 FPS
- **Tile Size**: 8 × 8 (Sprite alignment)
- **Level Width**: ~200 tiles
- **Entity Limits**: 32 enemies, 64 projectiles

## Runtime Architecture

The runtime operates on a memory arena system with three primary segments:
- **Global**: Persistent state across frames and levels.
- **Level**: Assets and state loaded for a single level.
- **Frame**: Temporary memory reset every tick.

## Features

### Text Rendering
- 3x5 pixel font supporting digits (0-9), uppercase letters (A-Z), and special characters (`:`, `/`, space)
- `FrameBuffer::draw_text()` for string rendering
- `FrameBuffer::draw_u32()` for fixed-width numeric display

### Sprite Rendering
- `FrameBuffer::draw_sprite()` for rendering arbitrary-size sprites (tile-aligned)
- `FrameBuffer::draw_tile()` decodes 16-byte GameBoy-style 2bpp planar tiles
- Supports transparency (color 0 is transparent)
- Automatic bounds checking for safe rendering
- Signed coordinate support for off-screen positioning
- Standalone asset pipeline: `tools/spritec` converts PNGs to binary `.2bpp` files

### Debug Overlay (Optional)
Enable with `--features debug_overlay`:
```bash
cargo run --features debug_overlay
```

Displays real-time metrics:
- **FPS**: Frames per second
- **G**: Global arena usage (bytes)
- **L**: Level arena usage (bytes)
- **F**: Frame arena usage (bytes)

## Documentation

### Concept Docs

Start with [Foundations](docs/concepts/00-foundations.md), then follow the numbered order:

| #   | Document                                                                       | Covers                                                                            |
| --- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- |
| 00  | [Foundations](docs/concepts/00-foundations.md)                                 | Bits, bytes, binary, memory, pixels, buffers, frames, ticks, sprites, bitwise ops |
| 01  | [Hardware-Constrained Design](docs/concepts/01-hardware-constrained-design.md) | Constraint philosophy, real hardware comparisons (Game Boy, NES, Atari)           |
| 02  | [Memory Arena](docs/concepts/02-memory-arena.md)                               | Arena/bump allocation, alignment, multi-arena partitioning                        |
| 03  | [Static Memory Patterns](docs/concepts/03-static-memory-patterns.md)           | Binary segments, `UnsafeCell`, `Global<T>` wrapper                                |
| 04  | [FrameBuffer & Bit-Packing](docs/concepts/04-framebuffer-bit-packing.md)       | Framebuffers, 2-bit pixel packing, set/get pixel                                  |
| 05  | [Sprite Rendering](docs/concepts/05-sprite-rendering.md)                       | Sprite data, draw_sprite algorithm, transparency, clipping                        |
| 06  | [Fixed Timestep](docs/concepts/06-fixed-timestep.md)                           | Game loops, determinism, tick-based time                                          |
| 07  | [Fixed-Capacity Entities](docs/concepts/07-fixed-capacity-entities.md)         | Object pooling, fixed arrays, entity budgets                                      |
| 08  | [Debug Overlays](docs/concepts/08-debug-overlays.md)                           | Bitmap fonts, conditional compilation, Cargo features                             |

### Architecture Decision Records

See [`docs/adr/`](docs/adr/) for the full set of ADRs documenting design decisions.