# Feature-Gated Debug Overlays

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially sections on pixels, framebuffers, and rendering. Also helpful: [FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md) for understanding how pixels are drawn.
>
> **Related ADR**: [0009 — Debug Overlay Feature](../adr/0009-debug-overlay-feature.md)

---

## What Is a Debug Overlay?

A **debug overlay** is information rendered directly on top of the game screen that helps the developer understand what's happening at runtime. Think of it like a heads-up display (HUD) for the developer — not for the player.

```
┌──────────────────────────────────────────┐
│ FPS:60 G:12/256 L:89/512 F:2/256         │  ← Debug overlay
│                                          │
│                                          │
│          Game content below...           │
│                                          │
│                                          │
└──────────────────────────────────────────┘
```

In the example above:
- **FPS:60** — the game is running at 60 frames per second (the target)
- **G:12/256** — the [Global Arena](02-memory-arena.md) has used 12 KB of its 256 KB capacity
- **L:89/512** — the [Level Arena](02-memory-arena.md) has used 89 KB of its 512 KB capacity
- **F:2/256** — the [Frame Arena](02-memory-arena.md) has used 2 KB of its 256 KB capacity

This tells you at a glance whether the game is hitting its performance target and how much memory headroom remains.

---

## Why Not Just Log to a File or Console?

### Problem 1: Volume

bit-bound's [game loop](06-fixed-timestep.md) runs 60 times per second. If you log debug info to a file or console output each tick, you generate **60 lines per second** — 3,600 lines per minute. That's unreadable. You can't watch text scroll by at that speed and extract meaningful information.

### Problem 2: Performance Impact

Writing to a file or console is an **I/O operation** — the program must communicate with the operating system, which may involve disk access, terminal rendering, or network output. This is slow compared to simply writing a few pixels to the [framebuffer](04-framebuffer-bit-packing.md). In a loop that must complete in 16.67ms, even 1ms of I/O per tick is a 6% performance hit.

### Problem 3: No Spatial Context

A log line like `Player at (50, 80)` tells you a number, but doesn't help you understand the visual result. An overlay draws the debug info **directly on the game screen**, in the same visual context as the game content. You see the FPS counter updating in real time, right next to the game action.

---

## What Is Feature Gating?

**Feature gating** means making a piece of code only exist in certain builds. The debug overlay is useful during development, but it should not exist in the final game — it wastes CPU cycles and screen space.

There are two approaches:

### Runtime Flag (Not What bit-bound Uses)

```rust
if DEBUG_ENABLED {
    render_debug_overlay();
}
```

This checks a boolean every tick. The code for `render_debug_overlay()` is always compiled into the binary, always takes up space, and the CPU evaluates the `if` condition every frame — even when the overlay is off.

### Compile-Time Feature Gate (What bit-bound Uses)

```rust
#[cfg(feature = "debug_overlay")]
{
    render_debug_overlay();
}
```

With `#[cfg(...)]`, the compiler **completely removes** the debug code when the feature is disabled. The code doesn't exist in the binary. No `if` check, no function, no overhead. It's as if the debug code was never written.

### How Cargo Features Work

In Rust's build system (Cargo), features are declared in `Cargo.toml`:

```toml
[features]
debug_overlay = []   # An empty feature — no dependencies, just a flag
```

You enable it at build time:

```bash
# Development: overlay enabled
cargo run --features debug_overlay

# Release: overlay stripped entirely
cargo run --release
# (or explicitly: cargo run without --features → overlay removed)
```

### What `#[cfg(...)]` Does

The `#[cfg(feature = "debug_overlay")]` attribute tells the compiler: "Only include this code if the `debug_overlay` feature is enabled." If the feature is off, the compiler skips the annotated code entirely — it's not parsed, not compiled, not included in the binary. This is called **conditional compilation**.

```rust
// This module only exists when the feature is enabled
#[cfg(feature = "debug_overlay")]
mod debug;

// In the game loop:
#[cfg(feature = "debug_overlay")]
{
    let info = debug::DebugInfo {
        fps: measured_fps,
        global_used: memory.global.used(),
        global_capacity: memory.global.capacity(),
        level_used: memory.level.used(),
        level_capacity: memory.level.capacity(),
        frame_used: memory.frame.used(),
        frame_capacity: memory.frame.capacity(),
    };
    debug::render_debug_overlay(&mut framebuffer, &info);
}
```

When `debug_overlay` is disabled, all of this code vanishes. The binary is smaller, the game loop is shorter, and there's zero performance cost. This connects to [Hardware-Constrained Design](01-hardware-constrained-design.md) — even development tools should respect the constraint philosophy.

---

## How Do You Draw Text Without a Font Engine?

In most high-level environments, text rendering is automatic — you provide a string and the framework handles font loading, glyph lookup, kerning, anti-aliasing, and layout. Thousands of lines of code run behind the scenes.

bit-bound has none of that. The screen is a 160×144 [framebuffer](04-framebuffer-bit-packing.md) of 2-bit pixels. To render text, we need a way to convert characters into pixel patterns — and we have to build it ourselves.

### What Is a Bitmap Font?

A **bitmap font** is the simplest possible font format: each character is represented as a small grid of pixels. No curves, no vectors, no anti-aliasing — just "this pixel is on" or "this pixel is off."

bit-bound uses a **3×5 pixel bitmap font** — each character is 3 pixels wide and 5 pixels tall:

```
Letter "F":        Letter "0":        Letter ":":

 ███               ░███░              ░░░░░
 █░░               █░░░█              ░░█░░
 ██░               █░░░█              ░░░░░
 █░░               █░░░█              ░░█░░
 █░░               ░███░              ░░░░░

 (3×5 pixels)      (3×5 pixels)       (3×5 pixels)
```

### How Glyph Data Is Stored

Each row of a character is 3 pixels. That fits in 3 bits. Five rows = five 3-bit values. These are stored as a small lookup table:

```rust
// The letter "F" as 5 bitmask rows:
// Row 0: ███ = 0b111 = 7
// Row 1: █░░ = 0b100 = 4
// Row 2: ██░ = 0b110 = 6
// Row 3: █░░ = 0b100 = 4
// Row 4: █░░ = 0b100 = 4

const FONT_F: [u8; 5] = [7, 4, 6, 4, 4];
```

The complete font table maps each supported character to its 5-row glyph:

```rust
fn glyph_for(ch: char) -> &[u8; 5] {
    match ch {
        '0' => &[7, 5, 5, 5, 7],
        '1' => &[2, 6, 2, 2, 7],
        // ...
        'F' => &[7, 4, 6, 4, 4],
        // ...
        ':' => &[0, 2, 0, 2, 0],
        ' ' => &[0, 0, 0, 0, 0],  // space = all blank
        _ => &[5, 2, 5, 2, 5],    // unknown character = checkerboard
    }
}
```

### How `draw_char` Works

To render a character at screen position (x, y):

```rust
fn draw_char(&mut self, x: usize, y: usize, ch: char, color: u8) {
    let glyph = glyph_for(ch);

    for row in 0..5 {        // 5 rows per character
        for col in 0..3 {    // 3 columns per character
            // Check if this pixel should be "on"
            // The bitmask is read from left to right:
            // bit 2 = leftmost pixel, bit 0 = rightmost
            if glyph[row] & (1 << (2 - col)) != 0 {
                let px = x + col;
                let py = y + row;

                // Bounds check (don't write outside the screen)
                if px < 160 && py < 144 {
                    let index = py * 160 + px;
                    self.set_pixel(index, color);
                    // set_pixel handles bit-packing (see FrameBuffer doc)
                }
            }
        }
    }
}
```

Let's trace the letter "F" being drawn at position (2, 0):

```
Glyph: [7, 4, 6, 4, 4]

Row 0: glyph[0] = 7 = 0b111
  col 0: bit 2 = 1 → draw pixel at (2, 0) ✓
  col 1: bit 1 = 1 → draw pixel at (3, 0) ✓
  col 2: bit 0 = 1 → draw pixel at (4, 0) ✓

Row 1: glyph[1] = 4 = 0b100
  col 0: bit 2 = 1 → draw pixel at (2, 1) ✓
  col 1: bit 1 = 0 → skip
  col 2: bit 0 = 0 → skip

Row 2: glyph[2] = 6 = 0b110
  col 0: bit 2 = 1 → draw pixel at (2, 2) ✓
  col 1: bit 1 = 1 → draw pixel at (3, 2) ✓
  col 2: bit 0 = 0 → skip

... and so on for rows 3 and 4.

Result on screen:
  (2,0)(3,0)(4,0)  →  ███
  (2,1)            →  █
  (2,2)(3,2)       →  ██
  (2,3)            →  █
  (2,4)            →  █
```

### Drawing Strings and Numbers

`draw_text()` calls `draw_char()` repeatedly, advancing the x position by 4 pixels per character (3 for the glyph + 1 for spacing):

```rust
fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u8) {
    for (i, ch) in text.chars().enumerate() {
        self.draw_char(x + i * 4, y, ch, color);
        //                  ^^^ 4-pixel advance: 3 for glyph + 1 for spacing
    }
}
```

`draw_u32()` converts a number to individual digit characters and draws each one. No heap-allocated `String` is created — the number is decomposed digit-by-digit using division and modulo:

```
draw_u32(18, 0, 60, color)  →  draws "6" at x=18, "0" at x=22
```

This is how the debug overlay renders `FPS:60 G:12/256 L:89/512 F:2/256` — it's just a sequence of `draw_text` and `draw_u32` calls, each rendering tiny 3×5 pixel characters directly into the [framebuffer](04-framebuffer-bit-packing.md).

---

## The Complete Debug Overlay Pipeline

Here's how everything connects, from data collection to visible pixels:

```
1. COLLECT — During the tick, gather metrics:
   ┌─────────────────────────────────────────────┐
   │ DebugInfo {                                  │
   │   fps: 60,          ← measured from timing   │
   │   global_used: 12,  ← from Global Arena      │
   │   level_used: 89,   ← from Level Arena       │
   │   frame_used: 2,    ← from Frame Arena       │
   │ }                                            │
   └─────────────────────────────────────────────┘
              │
              ▼
2. RENDER — Convert metrics to pixels:
   ┌─────────────────────────────────────────────┐
   │ draw_text(2, 0, "FPS:", 3)                   │
   │ draw_u32(18, 0, 60, 3)                       │
   │ draw_text(34, 0, "G:", 3)                    │
   │ draw_u32(42, 0, 12, 3)                       │
   │ ... etc for each metric                      │
   └─────────────────────────────────────────────┘
              │
              ▼
3. FRAMEBUFFER — Pixels are set via bit-packing:
   ┌─────────────────────────────────────────────┐
   │ set_pixel() for each "on" pixel in each     │
   │ glyph of each character of each label/value │
   │ (see FrameBuffer & Bit-Packing doc)         │
   └─────────────────────────────────────────────┘
              │
              ▼
4. DISPLAY — Framebuffer is presented to screen
   (debug overlay is part of the rendered frame,
    indistinguishable from game content)
```

---

## History

- **1980s — Hidden developer modes**: Early console games included hidden debug tools accessed via cheat codes or hardware switches. These were compiled into every cartridge — there was no conditional compilation.
- **1996 — Quake developer console**: PC games introduced the in-game developer console (`~` key) for real-time variable inspection. Always compiled in, just hidden from the player.
- **2000s — Profiling overlays**: Game engines added built-in displays for frame time, draw calls, and memory usage. Unity's Stats window, Unreal's `stat` commands.
- **Classic bitmap fonts**: Old computers had character ROMs — the Commodore 64's character ROM at address `$D000` held 8×8 pixel bitmaps for 256 characters. Game developers hand-drew even smaller pixel fonts for HUDs. bit-bound's 3×5 font follows this tradition at the smallest practical size.

## Alternatives

| Approach                                 | Description                                        | Trade-off                                                                             |
| ---------------------------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------- |
| **Runtime flag** (`if debug { ... }`)    | Toggle at runtime. Code always compiled in.        | Convenient, but binary includes unused code. Branch overhead each frame.              |
| **External profiler** (Tracy, PIX, perf) | Attach a profiling tool to the running process.    | Powerful (flame graphs, timelines), but heavyweight. Requires external tooling.       |
| **Log to file**                          | Write diagnostics to a file for offline analysis.  | Good for post-mortem debugging, but no real-time feedback. I/O overhead.              |
| **ImGui / Dear ImGui**                   | Industry-standard immediate-mode debug UI library. | Rich, interactive, GPU-rendered. But requires a graphics API and is far more complex. |
| **Separate debug window**                | Render debug info in a second OS window.           | Keeps game view clean, but needs windowing system integration.                        |

## Further Reading

- [Foundations — Bitwise Operations](00-foundations.md#14-bitwise-operations--the-programmers-screwdriver) — How the bitmask glyph rendering works
- [FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md) — How `set_pixel` stores the pixel data
- [Cargo — Features](https://doc.rust-lang.org/cargo/reference/features.html) — How Cargo feature flags work
- [The Rust Reference — Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html) — `#[cfg()]` attribute documentation
