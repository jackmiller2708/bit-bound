# Sprite Binary Format (v1)

This document defines the official binary format for sprite assets in BitBound.

## Specification

- **Tile Size**: 8x8 pixels.
- **Color Depth**: 2 bits per pixel (4 colors).
- **Encoding**: 2bpp planar (GameBoy style).
- **Transparency**: Index 0 is treated as transparent by the renderer.

## Data Layout

A sprite file is a raw stream of 8x8 tiles stored in row-major order (left-to-right, then top-to-bottom). There is no file header.

### Tile Encoding (2bpp Planar)

Each 8x8 tile occupies exactly **16 bytes**.
Each row of 8 pixels is represented by **2 bytes**:
1.  **Byte 0 (Low Plane)**: Bit 0 of the color index for each of the 8 pixels.
2.  **Byte 1 (High Plane)**: Bit 1 of the color index for each of the 8 pixels.

Pixels are stored **MSB-first** (bit 7 is the leftmost pixel, bit 0 is the rightmost).

```text
| Color Index | Binary | High Plane Bit | Low Plane Bit |
| ----------- | ------ | -------------- | ------------- |
| 0           | 00     | 0              | 0             |
| 1           | 01     | 0              | 1             |
| 2           | 10     | 1              | 0             |
| 3           | 11     | 1              | 1             |
```

### Padding Rules

Sprites whose dimensions are not multiples of 8 MUST be padded to the next 8-pixel boundary during conversion. The extra pixels should be filled with index 0 (transparent).

## Versioning

Current Version: **v1**
Status: **Stable**
