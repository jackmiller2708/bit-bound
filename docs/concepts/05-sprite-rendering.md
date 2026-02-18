# Sprite Rendering

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially the sections on pixels, framebuffers, and what a sprite is.
>
> **Related ADRs**:
> - [0010 — Sprite Rendering System (Deprecated)](../adr/0010-sprite-rendering-system.md)
> - [0011 — Binary Asset Pipeline](../adr/0011-binary-asset-pipeline.md)
> - [0012 — Tile-Based Sprite Rendering](../adr/0012-tile-based-sprite-rendering.md)

---

## What Is a Sprite?

In most high-level environments, if you want to put an image on screen, you load an image file (PNG, JPEG) and the framework handles everything — decoding the format, scaling, positioning, and painting the pixels to the display.

In bit-bound, none of that exists. There is no browser, no PNG decoder, no CSS, no layout engine. A sprite is something you have to understand from the ground up.

### A Sprite Is Just a Grid of Numbers

Imagine you have graph paper. Each square on the paper is one pixel. You number each square with a color:

```
Row 0:   0  0  3  3  3  3  0  0
Row 1:   0  3  1  1  1  1  3  0
Row 2:   3  1  2  1  1  2  1  3
Row 3:   3  1  1  1  1  1  1  3
Row 4:   3  1  1  2  2  1  1  3
Row 5:   3  1  1  1  1  1  1  3
Row 6:   0  3  1  1  1  1  3  0
Row 7:   0  0  3  3  3  3  0  0
```

Each number represents a color:
- `0` = transparent (don't draw anything — let the background show through)
- `1` = lightest color (like a skin or highlight)
- `2` = dark color (used for eyes, mouth, details)
- `3` = darkest color (outline)

When you "render" this with the palette, you get:

```
        ██████
      ██░░░░░░██
    ██░░▓▓░░▓▓░░██       ░ = lightest
    ██░░░░░░░░░░██       ▓ = dark (eyes/mouth)
    ██░░░░▓▓▓▓░░██       █ = darkest (outline)
    ██░░░░░░░░░░██
      ██░░░░░░██         (spaces are transparent)
        ██████
```

That's a face! A very small, 8×8 pixel face. This is exactly how game characters, enemies, items, and all visual elements were drawn on Game Boy, NES, and similar consoles.

### Where Did the Word "Sprite" Come From?

Engineers at Texas Instruments in the late 1970s coined the term. They were designing graphics chips that could move small images independently of the background. These little images seemed to "float" freely on the screen, like sprites (fairy-like creatures) in mythology. The name stuck.

### Sprites vs. Images in High-Level Environments

| Feature      | Typical Framework             | bit-bound (Sprites)                                     |
| ------------ | ----------------------------- | ------------------------------------------------------- |
| Format       | PNG, JPEG, SVG, WebP          | PNG source → binary `.2bpp` (via `spritec` tool)        |
| Colors       | 16.7 million+                 | 4                                                       |
| Loading      | Runtime decodes file          | Tool converts PNG → Binary, loaded via `include_bytes!` |
| Positioning  | Layout engine / draw calls    | Manual tile-based rendering                             |
| Transparency | Alpha channel (0–255 opacity) | Color 0 = transparent (binary: visible or not)          |
| Size         | Arbitrary, scaled by browser  | Padded to 8×8 tile multiples                            |

---

## How Sprite Data Is Stored (2bpp Planar)

While humans find flat grids easy to read, computers (especially constrained hardware) often use **bit-packing** to save space. In BitBound, we use the **GameBoy-style 2bpp planar format**.

### Why Not 1 Byte Per Pixel?

If we use one `u8` (8 bits) per pixel, but only have 4 colors (which fit in 2 bits), we are wasting 6 bits (75%) of every byte!

| Color Index | Binary | u8 Binary  | Wasted Space |
| ----------- | ------ | ---------- | ------------ |
| 0           | `00`   | `00000000` | 6 bits       |
| 3           | `11`   | `00000011` | 6 bits       |

By packing pixels together, we can store 4 pixels in a single byte (or 64 pixels in 16 bytes), cutting our asset size **in half**.

### Tile-Based Layout

Sprites are divided into **8×8 pixel tiles**. 
- A 16×16 sprite is a 2×2 grid of tiles.
- A 35×16 sprite is padded to 40×16 (a 5×2 grid of tiles).

Each tile is exactly **16 bytes**. The data is a raw stream of these 16-byte blocks, stored in row-major order (left-to-right, then top-to-bottom).

### Planar Bitplanes (The Technical Part)

Inside a single 16-byte tile, chaque row of 8 pixels is encoded using **2 bytes**. These are called "bitplanes":
1.  **Low Plane (Byte 0)**: Stores the low bit (bit 0) of the index for all 8 pixels.
2.  **High Plane (Byte 1)**: Stores the high bit (bit 1) of the index for all 8 pixels.

To get the color of the first pixel in a row:
- Look at bit 7 of the Low Plane byte.
- Look at bit 7 of the High Plane byte.
- Combine them: `(HighBit << 1) | LowBit`.

You can read more about the exact layout in [Asset Pipeline Specification](../sprite_format.md).

---

## The Sprite Struct

In our engine, a sprite isn't just a raw slice of bytes; it's a structured piece of metadata:

```rust
pub struct Sprite {
    pub width: usize,   // Actual width in pixels (e.g., 35)
    pub height: usize,  // Actual height in pixels (e.g., 16)
    pub tiles_x: usize, // Width in 8px tiles (e.g., 5)
    pub tiles_y: usize, // Height in 8px tiles (e.g., 2)
    pub data: &'static [u8], // The raw 2bpp binary data
}
```

---

## How a Sprite Gets Drawn Onto the Screen

The engine uses a two-step process to render complex sprites.

### Step 1: `draw_tile`

The core rendering primitive is `draw_tile`. It decodes the complex "planar" format for a single 8×8 block and stamps it on the screen:

```rust
pub fn draw_tile(&mut self, x: i32, y: i32, tile_data: &[u8]) {
    for row in 0..8 {
        // Read the two bitplanes for this row
        let low = tile_data[row * 2];
        let high = tile_data[row * 2 + 1];

        for col in 0..8 {
            // Combine bits to get the color index (0-3)
            let bit = 7 - col;
            let color = ((low >> bit) & 1) | (((high >> bit) & 1) << 1);

            // Skip transparent (index 0) and clip to screen
            if color != 0 {
                self.set_pixel(x + col, y + row, color);
            }
        }
    }
}
```

### Step 2: `draw_sprite`

The high-level `draw_sprite` method simply iterates over the sprite's tile grid and calls `draw_tile` for each one:

```rust
pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite) {
    for ty in 0..sprite.tiles_y {
        for tx in 0..sprite.tiles_x {
            // Find where this tile starts in the binary data
            let offset = (ty * sprite.tiles_x + tx) * 16;
            let tile_data = &sprite.data[offset..offset + 16];

            // Render the tile at the correct offset
            self.draw_tile(x + (tx * 8), y + (ty * 8), tile_data);
        }
    }
}
```

### Tracing One Pixel

Let's trace what happens when we draw the sprite at position (50, 30) and process the **left eye** at sprite position (col=2, row=2):

```
1. Read color: sprite[2 * 8 + 2] = sprite[18] = 2 (dark color)

2. Is it transparent? color == 0? No (it's 2). Continue.

3. Screen position:
   screen_x = 50 + 2 = 52
   screen_y = 30 + 2 = 32

4. Bounds check:
   52 >= 0 and 52 < 160? ✓
   32 >= 0 and 32 < 144? ✓

5. Write to framebuffer:
   pixel_index = 32 * 160 + 52 = 5,172
   set_pixel(5172, 2)
   → byte_index = 5172 / 4 = 1293
   → bit_offset = (5172 % 4) * 2 = 0
   → buffer[1293] gets color 2 written to bits 0–1
```

That one pixel is now in the framebuffer. The renderer does this for all 64 pixels in the sprite. The non-transparent ones (about 36 in this face sprite) get written; the transparent ones (the corners) are skipped.

---

## Transparency: Why Color 0 Is Special

In modern graphics, transparency uses an **alpha channel** — a number from 0 (fully transparent) to 255 (fully opaque). This requires an extra byte per pixel (RGBA = 4 bytes total).

In bit-bound, we can't afford an extra byte. Instead, we use **color keying**: one specific color value (0, the lightest color) is designated as "transparent." When the renderer encounters color 0, it simply doesn't draw that pixel.

```
Sprite data:                     What gets drawn:

0  0  3  3  3  3  0  0           ·  ·  █  █  █  █  ·  ·
0  3  1  1  1  1  3  0           ·  █  ░  ░  ░  ░  █  ·
3  1  2  1  1  2  1  3           █  ░  ▓  ░  ░  ▓  ░  █

The 0s are NOT drawn — whatever was previously in the
framebuffer at those positions remains visible.
```

This is how you layer sprites on top of backgrounds: the background is drawn first, then sprites are drawn on top. The transparent pixels in the sprite let the background show through.

### Limitations

Color 0 can never appear as a visible color in a sprite. If you want a pixel to be the lightest color, you have to rethink your design. This is a real artistic constraint that Game Boy artists dealt with — transparency and the lightest color were the same thing. This is also covered in [Hardware-Constrained Design](01-hardware-constrained-design.md).

---

## Clipping: What Happens at Screen Edges

Imagine a character walking off the right side of the screen. Half the sprite is visible, half is beyond the screen edge. In most graphics frameworks, clipping is handled automatically. In bit-bound, you must handle it yourself.

### The Problem Without Clipping

If the sprite is at position x=156 and it's 8 pixels wide, pixels at columns 160–163 would be **off screen**. Without clipping, writing to pixel_index `y * 160 + 160` would actually write to the first pixel of the **next row** — corrupting the image. Or worse, writing beyond the framebuffer's 5,760 bytes would crash the program.

### The Solution: Per-Pixel Bounds Checking

The `draw_sprite` code checks every pixel:

```rust
if screen_x < 0 || screen_x >= 160 { continue; }
if screen_y < 0 || screen_y >= 144 { continue; }
```

This means:
- A sprite at x=156 draws columns 0–3 (at screen x 156–159) but skips columns 4–7 (at screen x 160–163)
- A sprite at x=−4 draws columns 4–7 (at screen x 0–3) but skips columns 0–3 (at screen x −4 to −1)

```
Screen (160px wide):
┌─────────────────────────────────────────┐
│                                         │
│                                   ┌─────┤─ ─ ─ ┐
│     visible portion ───────────►  │██░░░│░ ░ ░ │
│                                   │██░░░│░ ░ ░ │ ← off-screen
│                                   │██░░░│░ ░ ░ │   (skipped)
│                                   └─────┤─ ─ ─ ┘
│                                         │
└─────────────────────────────────────────┘
```

### Why Signed Coordinates (`i32`)?

The sprite position uses `i32` (a signed 32-bit integer that can be negative) instead of `usize` (an unsigned integer that can't be negative — see [Foundations](00-foundations.md#1-bits-and-bytes--the-smallest-units) for integer types). This is because sprites legitimately have negative positions — a character entering from the left side of the screen starts at a negative x value and moves rightward until fully visible.

---

## How Sprites Become Characters on Screen

Let's connect the dots from a PNG file on disk to what the player actually sees:

```
1. DESIGN TIME: Artist exports sprites as PNG files using the 4-color palette.
   Placed in assets/raw/

2. CONVERSION: Developer runs `cargo run -p spritec`.
   The tool reads PNGs, validates palette, pads to 8px tiles, and encodes as 2bpp planar binary.
   Output written to assets/processed/spaceship_0.2bpp

3. COMPILATION: The game layer loads the binary using `include_bytes!`.
   No generated source code; just raw bytes baked into the final executable.

4. EACH FRAME: Game logic selects the current animation frame and calls `draw_sprite`.
   
5. RENDERING: `draw_sprite` iterates over tiles, and `draw_tile` decodes the bitplanes
   row-by-row, writing pixels to the framebuffer.

   Index 0 → (155, 188, 15)  lightest green
   Index 1 → (139, 172, 15)  light green
   Index 2 → (48, 98, 48)    dark green
   Index 3 → (15, 56, 15)    darkest green
              │
              ▼
7. DISPLAY: Your monitor shows a tiny green spaceship on screen
```

This entire process happens **60 times per second**. Every frame, the framebuffer is cleared and everything is redrawn from scratch. The PNG-to-array conversion (steps 1–3) only happens once at build time — at runtime, sprites are just static byte arrays with zero loading cost.

---

## History

- **1973 — Xerox Alto**: One of the first systems to implement `BitBLT` (Bit Block Transfer) — copying a rectangular block of pixels from one memory region to another. This is the foundational operation behind sprite rendering.
- **1977 — TMS9918 (TI)**: The first dedicated sprite hardware chip. It handled sprite positioning, movement, and collision detection automatically, freeing the CPU from pixel-level work.
- **1983 — NES PPU**: Supported 64 sprites stored in OAM (Object Attribute Memory). The hardware composited sprites over the background tilemap with priority, flipping, and palette selection. Famous limitation: maximum 8 sprites per scanline (causing the flickering you may have seen in old NES games).
- **1989 — Game Boy**: 40 sprites, 8×8 or 8×16 pixels, 4 shades. The direct inspiration for bit-bound's sprite system.
- **1990s — Software rendering era**: PC games without sprite hardware (Doom, StarCraft) implemented sprite rendering entirely in CPU code — exactly like bit-bound does.
- **2000s–present**: GPUs render "sprites" as textured rectangles using shaders. The concept is the same (small image stamped at a position), but the implementation is completely different.

## Alternatives

| Approach                             | Description                                                                  | Trade-off                                                                                       |
| ------------------------------------ | ---------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| **Tile-based rendering**             | Screen is a fixed grid of tile indices. No free positioning.                 | Very memory-efficient for backgrounds, but sprites can't be placed at arbitrary pixel positions |
| **Hardware sprites** (NES, Game Boy) | Dedicated silicon handles compositing                                        | Zero CPU cost, but hard sprite count limits (8 per scanline on NES)                             |
| **Pre-clipped blitting**             | Calculate visible rectangle first, then copy without per-pixel bounds checks | Faster (fewer branches), but more complex clipping math                                         |
| **GPU textured quads**               | Upload image as a GPU texture, draw a rectangle                              | Supports rotation, scaling, alpha blending — but requires a graphics API (OpenGL/Vulkan)        |
| **Sprite sheets**                    | Pack many sprites into one large image, reference each by coordinates        | Standard in web gamedev (`background-position` in CSS!). Reduces texture switches on GPU        |

## Further Reading

- [Foundations — What Is a Sprite?](00-foundations.md#13-what-is-a-sprite) — The prerequisite explanation
- [FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md) — How `set_pixel` stores the color data
- [NES Dev Wiki — PPU OAM](https://www.nesdev.org/wiki/PPU_OAM) — How the NES hardware managed 64 sprites
- [Rodrigo Copetti — "Game Boy Architecture"](https://www.copetti.org/writings/consoles/game-boy/) — Sprite rendering on the actual Game Boy
