# 6. FrameBuffer Bit-Packing Implementation

Date: 2026-02-17

## Status
Accepted

Amends [4. Hardware Constraints](0004-hardware-constraints.md)

## Context
ADR 0004 established the 160x144 resolution and 4-color palette. To minimize memory usage and match classic hardware behavior, we need a specific strategy for storing these pixels.

## Decision
The `FrameBuffer` will use a bit-packed array of bytes.
- 4 pixels are packed into a single `u8`.
- Each pixel occupies 2 bits.
- Pixel index to byte/bit mapping:
  - `byte_index = index / 4`
  - `bit_offset = (index % 4) * 2`
- Colors are masked and shifted into place.

## Consequences
- **Positive**: Minimal memory footprint (5,760 bytes).
- **Negative**: Slight CPU overhead for bitwise operations during `set_pixel` and `get_pixel`.
