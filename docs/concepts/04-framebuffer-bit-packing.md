# FrameBuffer & Bit-Packing

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially sections on bits/bytes, buffers, pixels, frames, and bitwise operations.
>
> **Related ADRs**: [0004 — Hardware Constraints](../adr/0004-hardware-constraints.md), [0006 — FrameBuffer Bit-Packing](../adr/0006-framebuffer-bit-packing.md)

---

## What Is a FrameBuffer?

Let's break the word apart:

- **Frame**: One complete image shown on screen. At 60 FPS, the screen displays 60 frames per second.
- **Buffer**: A chunk of memory used to temporarily hold data between two operations.

A **framebuffer** is the chunk of memory that holds all the pixel data for one complete frame. Your game logic writes pixels into this memory, and the display system reads from it to show the image on screen.

### A Familiar Analogy

If you've ever used an HTML Canvas or a similar 2D graphics API, you've already interacted with a framebuffer:

```javascript
const canvas = document.getElementById("game");
const ctx = canvas.getContext("2d");

// This ImageData IS a framebuffer — a raw array of pixel values
const imageData = ctx.createImageData(160, 144);
// imageData.data is a Uint8ClampedArray of RGBA bytes
// imageData.data[0] = red value of pixel (0,0)
// imageData.data[1] = green value of pixel (0,0)
// imageData.data[2] = blue value of pixel (0,0)
// imageData.data[3] = alpha value of pixel (0,0)
// imageData.data[4] = red value of pixel (1,0)   ← next pixel
// ...

ctx.putImageData(imageData, 0, 0);  // Display the framebuffer on screen
```

In a typical 2D graphics context, each pixel takes **4 bytes** (RGBA). In bit-bound, each pixel takes **2 bits** — that's 16× less memory. The concept is identical, just the encoding is different.

### Why Not Just Use 32-bit RGBA?

bit-bound is designed to emulate a 1989 Game Boy-era console. Those consoles didn't have 32-bit color. They had 4 shades and 2 bits per pixel. Using the same representation:

1. **Enforces the constraint** — you literally can't store more than 4 colors
2. **Saves precious memory** — 5,760 bytes instead of 92,160 bytes
3. **Matches the mental model** — you think in terms of 4 colors, not millions

---

## What Is Bit-Packing?

### The Problem

bit-bound uses 4 colors, so each pixel needs only 2 bits. But the smallest unit of memory a computer can address is a **byte** (8 bits). If you use one byte per pixel, you waste 6 bits per pixel:

```
One byte per pixel (wasteful):

  Byte 0:  [0 0 0 0 0 0 | 1 1]   ← pixel 0 = color 3, but 6 bits are wasted
  Byte 1:  [0 0 0 0 0 0 | 0 1]   ← pixel 1 = color 1, 6 bits wasted
  Byte 2:  [0 0 0 0 0 0 | 1 0]   ← pixel 2 = color 2, 6 bits wasted
  Byte 3:  [0 0 0 0 0 0 | 0 0]   ← pixel 3 = color 0, 6 bits wasted

  Total: 4 bytes for 4 pixels (75% wasted!)
```

### The Solution: Pack Multiple Pixels Into One Byte

Since each pixel only needs 2 bits, you can fit **4 pixels** in a single byte (8 ÷ 2 = 4):

```
Bit-packed (efficient):

  One byte: [ P3 | P2 | P1 | P0 ]
             ──── ──── ──── ────
  Bits:       7 6  5 4  3 2  1 0

  Example:  [ 0 0 | 1 0 | 0 1 | 1 1 ]
              P3=0  P2=2  P1=1  P0=3

  Total: 1 byte for 4 pixels (0% wasted!)
```

This is **bit-packing** — squeezing multiple values into a single byte by using only the bits you need.

### Memory Savings Visualized

```
160 × 144 pixels = 23,040 total pixels

Without bit-packing (1 byte per pixel):
  23,040 bytes = 22.5 KB

With bit-packing (4 pixels per byte):
  23,040 ÷ 4 = 5,760 bytes = 5.6 KB

Savings: 17,280 bytes — that's 17 KB saved. In a 1 MB budget, this matters.
```

---

## How Bit-Packing Works — Step by Step

### The Mapping Formula

Given a pixel's index (0 to 23,039), you need to figure out:
1. **Which byte** does this pixel live in?
2. **Which 2 bits** within that byte belong to this pixel?

```
byte_index = pixel_index / 4        (integer division)
bit_offset = (pixel_index % 4) * 2  (remainder tells which slot)
```

Let's trace through some examples:

| Pixel Index | byte_index (÷4) | bit_offset ((mod 4)×2) | Position in Byte      |
| ----------- | --------------- | ---------------------- | --------------------- |
| 0           | 0               | 0                      | Bits 0–1              |
| 1           | 0               | 2                      | Bits 2–3              |
| 2           | 0               | 4                      | Bits 4–5              |
| 3           | 0               | 6                      | Bits 6–7              |
| 4           | 1               | 0                      | Bits 0–1 (next byte!) |
| 5           | 1               | 2                      | Bits 2–3              |

Pixels 0–3 share byte 0. Pixels 4–7 share byte 1. And so on.

### Writing a Pixel (set_pixel)

Let's say you want to set **pixel 5** to **color 2** (binary: `10`).

**Step 1: Find the byte and bit offset**
```
pixel_index = 5
byte_index  = 5 / 4 = 1
bit_offset  = (5 % 4) * 2 = 1 * 2 = 2
```
Pixel 5 is in byte 1, at bits 2–3.

**Step 2: Clear the old value at that position**

We need a mask that clears only bits 2–3, leaving everything else untouched:

```
Mask:  0b11 << 2  =  0b00001100   (the two bits we want to target)
NOT:  !0b00001100  =  0b11110011   (inverted — clears our target, keeps rest)

byte[1] &= 0b11110011    (bitwise AND — zeros out bits 2–3)
```

If byte[1] was `0b01011011` before:
```
  0b01011011
& 0b11110011
  ──────────
  0b01010011    ← bits 2–3 are now cleared to 00
```

**Step 3: Write the new color**

```
color = 2 = 0b10

Shifted:  0b10 << 2  =  0b00001000   (color placed at bits 2–3)

byte[1] |= 0b00001000    (bitwise OR — writes color without touching other bits)
```

```
  0b01010011
| 0b00001000
  ──────────
  0b01011011    ← bits 2–3 now hold 10 (color 2)
```

### Reading a Pixel (get_pixel)

Let's read **pixel 5** back:

**Step 1: Find the byte and bit offset** (same as writing)
```
byte_index = 1, bit_offset = 2
```

**Step 2: Shift the bits down**
```
byte[1] >> 2    shifts bits 2–3 into bits 0–1

  0b01011011 >> 2  =  0b00010110
```

**Step 3: Mask off everything except the lowest 2 bits**
```
  0b00010110
& 0b00000011
  ──────────
  0b00000010  =  2    ← 

The color is 2.
```

### The Complete Implementation

```rust
pub struct FrameBuffer {
    buffer: [u8; 5_760],  // 23,040 pixels ÷ 4 pixels per byte
}

impl FrameBuffer {
    /// Write a color (0–3) to a pixel position
    pub fn set_pixel(&mut self, index: usize, color: u8) {
        let byte_index = index / 4;
        let bit_offset = (index % 4) * 2;

        // Step 1: Clear the 2-bit slot (set to 00)
        self.buffer[byte_index] &= !(0b11 << bit_offset);

        // Step 2: Write the color into the slot
        self.buffer[byte_index] |= (color & 0b11) << bit_offset;
    }

    /// Read the color (0–3) at a pixel position
    pub fn get_pixel(&self, index: usize) -> u8 {
        let byte_index = index / 4;
        let bit_offset = (index % 4) * 2;

        // Shift down and mask to get the 2-bit value
        (self.buffer[byte_index] >> bit_offset) & 0b11
    }
}
```

### Why `color & 0b11`?

The `& 0b11` in `set_pixel` is a safety measure. If someone accidentally passes a color value of 5 (`0b101`), the mask clips it to 1 (`0b01`), preventing it from overwriting neighboring pixels' bits. It ensures only the lowest 2 bits are used.

---

## From Pixel Index to Screen Coordinates

The framebuffer is a **flat**, one-dimensional array. But the screen is a **2D grid** (160 columns × 144 rows). You need to convert between the two:

### 2D → 1D (When Writing)

```
pixel_index = y * SCREEN_WIDTH + x
```

Example: pixel at column 10, row 5:
```
pixel_index = 5 * 160 + 10 = 810
```

### 1D → 2D (When Reading)

```
x = pixel_index % SCREEN_WIDTH
y = pixel_index / SCREEN_WIDTH
```

Example: pixel index 810:
```
x = 810 % 160 = 10
y = 810 / 160 = 5
```

### Why Flat Instead of 2D?

In many languages, you might represent this as a 2D array:

```
pixels[row][col] = 2;  // Row 5, Column 10, Color 2
```

But this creates 144 separate arrays, each at a different memory address. The CPU can't efficiently read scattered data. A flat array is one contiguous block of memory — the CPU can read through it sequentially, which is **dramatically faster** (cache-friendly).

---

## 2D to Screen: The Full Pipeline

Let's trace the complete lifecycle of setting a pixel at position (10, 5) to color 3:

```
Step 1: Calculate pixel index
         pixel_index = 5 * 160 + 10 = 810

Step 2: Calculate byte and bit positions
         byte_index = 810 / 4 = 202
         bit_offset = (810 % 4) * 2 = 2 * 2 = 4

Step 3: Clear bits 4–5 of byte 202
         buffer[202] &= !(0b11 << 4)
         buffer[202] &= 0b11001111

Step 4: Write color 3 (0b11) at bits 4–5
         buffer[202] |= (0b11 << 4)
         buffer[202] |= 0b00110000

Step 5: At the end of the frame, the output system reads
        the entire buffer and converts each 2-bit value
        to an actual RGB color using the palette lookup:

        Index 0 → RGB(155, 188, 15)   lightest
        Index 1 → RGB(139, 172, 15)   light
        Index 2 → RGB(48, 98, 48)     dark
        Index 3 → RGB(15, 56, 15)     darkest   ← this one

Step 6: The screen displays a dark green pixel at row 5, column 10.
```

---

## History

### Why Was Bit-Packing Invented?

Bit-packing wasn't "invented" — it was the only option. When your entire system had a few kilobytes of memory, you couldn't waste a single bit.

- **1981 — IBM CGA**: The first color graphics card for PCs. 320×200 resolution, 4 colors, 2 bits per pixel — exactly bit-bound's approach. The framebuffer was 16,000 bytes. This was a significant chunk of the PC's 640 KB of RAM.
- **1989 — Game Boy**: 160×144, 4 shades, 2 bits per pixel. The framebuffer equivalent was part of the VRAM (Video RAM). The Game Boy is the direct inspiration for bit-bound's display system.

As memory became cheaper (megabytes, then gigabytes), bit-packing became unnecessary for mainstream displays. Modern GPUs use 32 bits per pixel natively. But bit-packing lives on in:

- **Compression formats** (PNG, JPEG, WebP use bit-level packing internally)
- **Network protocols** (TCP flags are bit-packed into header bytes)
- **Embedded systems** (IoT devices, microcontrollers with limited RAM)
- **Retro/fantasy console projects** (like bit-bound)

## Alternatives

| Approach                         | Memory/pixel | Description                                                 | Trade-off                                                       |
| -------------------------------- | ------------ | ----------------------------------------------------------- | --------------------------------------------------------------- |
| **2-bit packed** (bit-bound)     | 0.25 bytes   | 4 pixels per byte, bitwise math                             | Minimal memory, but CPU overhead for bit manipulation           |
| **8-bit indexed**                | 1 byte       | One byte per pixel, 256-color palette                       | Simple addressing, richer colors, 4× more memory                |
| **Planar graphics** (Amiga, EGA) | Varies       | Each bit of a pixel's color is stored in a separate "plane" | Hardware-efficient for palette tricks, very complex in software |
| **16-bit color**                 | 2 bytes      | 65,536 colors (5-6-5 RGB split)                             | Good balance of color and memory                                |
| **32-bit RGBA**                  | 4 bytes      | 16.7M colors + transparency                                 | The modern default. Simple. 16× the memory of 2-bit packed      |

## Further Reading

- [Foundations — Bits, Bytes, and Bitwise Operations](00-foundations.md) — The prerequisite knowledge for this document
- [Game Boy Pan Docs — LCD](https://gbdev.io/pandocs/LCDC.html) — How the actual Game Boy hardware stored and rendered pixels
- [Rodrigo Copetti — "Game Boy Architecture"](https://www.copetti.org/writings/consoles/game-boy/) — Beautiful breakdown of the Game Boy's PPU and tile/pixel system
- [IBM CGA — Technical Reference](https://www.seasip.info/VintagePC/cga.html) — The first mainstream 2-bpp framebuffer on PCs
