# Sprite Rendering

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially the sections on pixels, framebuffers, and what a sprite is.
>
> **Related ADR**: [0010 — Sprite Rendering System](../adr/0010-sprite-rendering-system.md)

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

| Feature      | Typical Framework             | bit-bound (Sprites)                            |
| ------------ | ----------------------------- | ---------------------------------------------- |
| Format       | PNG, JPEG, SVG, WebP          | Raw byte array                                 |
| Colors       | 16.7 million+                 | 4                                              |
| Loading      | Runtime decodes file          | Compiled into the program binary               |
| Positioning  | Layout engine / draw calls    | Manual pixel-by-pixel copy                     |
| Transparency | Alpha channel (0–255 opacity) | Color 0 = transparent (binary: visible or not) |
| Size         | Arbitrary, scaled by browser  | Fixed, pixel-perfect                           |

---

## How Sprite Data Is Stored

### Row-Major Order

The sprite grid above has 8 columns and 8 rows = 64 pixels. These are stored as a **flat, one-dimensional array** in memory (not a 2D grid). The pixels are listed **row by row**, left to right, top to bottom. This is called **row-major order** (see [FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md) for more on flat vs. 2D arrays):

```rust
const FACE: [u8; 64] = [
    // Row 0 (top row)
    0, 0, 3, 3, 3, 3, 0, 0,
    // Row 1
    0, 3, 1, 1, 1, 1, 3, 0,
    // Row 2 (eyes)
    3, 1, 2, 1, 1, 2, 1, 3,
    // Row 3
    3, 1, 1, 1, 1, 1, 1, 3,
    // Row 4 (mouth)
    3, 1, 1, 2, 2, 1, 1, 3,
    // Row 5
    3, 1, 1, 1, 1, 1, 1, 3,
    // Row 6
    0, 3, 1, 1, 1, 1, 3, 0,
    // Row 7 (bottom row)
    0, 0, 3, 3, 3, 3, 0, 0,
];

const FACE_WIDTH: usize = 8;
const FACE_HEIGHT: usize = 8;
```

### Why Is It Flat?

In many languages, you'd naturally use a 2D array (an array of arrays). But in memory, a 2D array is typically an array of pointers to other arrays — the rows could be scattered across different memory locations. A flat array is one **contiguous block** in memory, which is faster for the CPU to read sequentially (it can prefetch the next bytes because they're adjacent).

To access pixel (col, row) in a flat array:

```
index = row * width + col
```

For example, the right eye at column 5, row 2:
```
index = 2 * 8 + 5 = 21
FACE[21] = 2    ← yep, that's the dark "eye" color
```

### Why `u8` Instead of 2 Bits?

Wait — if we only have 4 colors (0–3), and those fit in 2 bits, why is each pixel a full `u8` (8 bits)?

Because the sprite data is a **source format** that's easy to author and understand. The bit-packing happens in the **framebuffer** (where memory savings matter). The sprite is a compile-time constant — it's baked into the binary and never changes. The small memory overhead of using `u8` per pixel in the source format doesn't matter much, and it makes the code dramatically easier to read:

```rust
// This is readable:
0, 3, 1, 1, 1, 1, 3, 0,

// This would be unreadable (4 pixels packed per byte):
0b01_11_01_00, 0b00_01_11_01,
```

The conversion to bit-packed format happens when `set_pixel` writes each sprite pixel into the framebuffer.

---

## How a Sprite Gets Drawn Onto the Screen

When you call `draw_sprite(x, y, &FACE, 8, 8)`, the renderer needs to copy every non-transparent pixel from the sprite array into the framebuffer at the correct position.

### The Full Algorithm — Step by Step

```rust
pub fn draw_sprite(
    &mut self,
    x: i32,           // Where to place the sprite on screen (column)
    y: i32,           // Where to place the sprite on screen (row)
    sprite: &[u8],    // The sprite pixel data
    width: usize,     // Sprite width in pixels
    height: usize,    // Sprite height in pixels
) {
    // Loop through every pixel in the sprite
    for row in 0..height {
        for col in 0..width {
            // --- Step 1: Read the color from the sprite data ---
            let color = sprite[row * width + col];

            // --- Step 2: Skip transparent pixels ---
            if color == 0 {
                continue;  // Don't draw anything — leave the background visible
            }

            // --- Step 3: Calculate screen position ---
            let screen_x = x + col as i32;
            let screen_y = y + row as i32;

            // --- Step 4: Clip to screen bounds ---
            if screen_x < 0 || screen_x >= 160 {
                continue;  // Off the left or right edge — skip
            }
            if screen_y < 0 || screen_y >= 144 {
                continue;  // Off the top or bottom edge — skip
            }

            // --- Step 5: Write to the framebuffer ---
            let pixel_index = screen_y as usize * 160 + screen_x as usize;
            self.set_pixel(pixel_index, color);
            // set_pixel handles the bit-packing internally
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

Let's connect the dots from raw sprite data to what the player actually sees:

```
1. COMPILE TIME: Sprite data exists as a const byte array in the source code
   ┌────────────────────────┐
   │ const PLAYER: [u8; 64] │
   │ = [0,0,3,3,3,3,0,0,   │
   │    0,3,1,1,1,1,3,0,   │
   │    ...]                │
   └────────────────────────┘
              │
              ▼
2. EACH FRAME: Game logic determines where to draw the sprite
   ┌──────────────────────────────────────┐
   │ let player_x = 50;                   │
   │ let player_y = 100;                  │
   │ fb.draw_sprite(player_x, player_y,   │
   │                &PLAYER, 8, 8);       │
   └──────────────────────────────────────┘
              │
              ▼
3. RENDERING: draw_sprite copies each non-transparent pixel
   into the framebuffer (5,760 bytes of bit-packed memory)
   ┌──────────────────────────────────┐
   │  Framebuffer: [u8; 5760]         │
   │  ┌──┬──┬──┬──┬──┬──┬──┬──┬──┐  │
   │  │░░│░░│░░│██│░░│░░│░░│░░│░░│  │
   │  │  ....  player pixels ....  │  │
   │  └──┴──┴──┴──┴──┴──┴──┴──┴──┘  │
   └──────────────────────────────────┘
              │
              ▼
4. OUTPUT: The output adapter reads the framebuffer and maps
   each 2-bit color index to an actual RGB color for the monitor

   Index 0 → (155, 188, 15)  lightest green
   Index 1 → (139, 172, 15)  light green
   Index 2 → (48, 98, 48)    dark green
   Index 3 → (15, 56, 15)    darkest green
              │
              ▼
5. DISPLAY: Your monitor shows a tiny green face on screen
```

This entire process happens **60 times per second**. Every frame, the framebuffer is cleared and everything is redrawn from scratch.

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
