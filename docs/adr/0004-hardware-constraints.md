# 4. Hardware Constraints

Date: 2026-02-17

## Status
Accepted

## Context
To foster architectural discipline, we need hard limits on output. Modern hardware is too forgiving; limiting the rendering capabilities forces intentionality in asset design and memory usage.

## Decision
We will enforce the following hardware limits:
- **Resolution**: 160 × 144 pixels. This matches popular handheld consoles from the early 90s.
- **Color Depth**: 4-color palette (2 bits per pixel).
- **Framebuffer**: The total VRAM/Framebuffer is limited to 5,760 bytes (160 * 144 / 4 pixels per byte).
- **Bit Packing**: To optimize memory, 4 pixels are packed into a single byte. Each pixel uses 2 bits, with offsets calculated as `(index % 4) * 2`.
- **Tile Size**: Standardized at 16 x 16 pixels.

## Consequences
- **Positive**: Smaller asset sizes, reduced memory footprint, and a cohesive "retro" aesthetic.
- **Negative**: Requires custom rendering logic (blitting, palette mapping) as modern graphics APIs usually expect 32-bit RGBA.
