# Foundations: Low-Level Computing from the Ground Up

> **Read this first.** This document explains the building blocks that every other concept doc in this folder relies on. If terms like "bits," "bytes," "framebuffer," or "bitwise operations" are unfamiliar, start here. Everything is explained from scratch — no prior low-level knowledge is assumed.

---

## Table of Contents

1. [Bits and Bytes — The Smallest Units](#1-bits-and-bytes--the-smallest-units)
2. [Binary and Hexadecimal — Counting Differently](#2-binary-and-hexadecimal--counting-differently)
3. [Memory — A Giant Numbered Grid](#3-memory--a-giant-numbered-grid)
4. [The Stack, the Heap, and Static Memory](#4-the-stack-the-heap-and-static-memory)
5. [What Is a Pixel?](#5-what-is-a-pixel)
6. [How Screens Display Images](#6-how-screens-display-images)
7. [How Computers Represent Colors](#7-how-computers-represent-colors)
8. [What Is a Buffer?](#8-what-is-a-buffer)
9. [What Is a Frame?](#9-what-is-a-frame)
10. [What Is a Framebuffer?](#10-what-is-a-framebuffer)
11. [What Is Rendering?](#11-what-is-rendering)
12. [What Is a Tick?](#12-what-is-a-tick)
13. [What Is a Sprite?](#13-what-is-a-sprite)
14. [Bitwise Operations — The Programmer's Screwdriver](#14-bitwise-operations--the-programmers-screwdriver)

---

## 1. Bits and Bytes — The Smallest Units

### What Is a Bit?

A **bit** is the smallest possible piece of information a computer can store. It has exactly two possible values: `0` or `1`. That's it. Every piece of data in a computer — every image, every string, every number, every function — is ultimately stored as a sequence of bits.

Think of a bit like a light switch: it's either **off** (`0`) or **on** (`1`).

```
A single bit:   0    or    1
                off        on
```

### Why Only 0 and 1?

Computers are electronic machines. At the hardware level, they work with electrical voltages. It's extremely reliable to distinguish between two states — "high voltage" vs "low voltage" (or "current flowing" vs "no current"). Distinguishing between 10 different voltage levels would be error-prone. So the entire foundation of computing is built on this binary (two-state) system.

### A Bit of History: Why Binary Won

This wasn't always obvious. Early computing pioneers had a choice: why not use 10 voltage levels (one per decimal digit) instead of just 2?

The answer comes down to the physical components available at the time. The earliest electronic computers — machines like ENIAC (1945) and Colossus (1943) — were **room-sized behemoths** built from **vacuum tubes**: glass cylinders the size of a light bulb, each acting as an electronic switch. ENIAC alone used **17,468 vacuum tubes**, weighed 30 tons, and consumed 150 kilowatts of power — enough to dim the lights in an entire city block.

```
A vacuum tube (actual size ~8 cm / 3 inches):

    ┌─────────┐
    │  ┌───┐  │
    │  │ ● │  │     ← glass envelope with heated
    │  │ │ │  │       filament and metal plates
    │  └─┬─┘  │
    │    │    │     Each one = one electronic switch
    └────┴────┘     (roughly equivalent to one modern transistor)
    ┌──┴──┴──┐
    └────────┘      ← metal pins for electrical connections
```

These tubes were **unreliable** — they burned out frequently, ran extremely hot, and their electrical characteristics drifted over time. Asking a vacuum tube to distinguish between 10 precise voltage levels would have been disastrous: a tube drifting even slightly would misread a "7" as a "6" or an "8," corrupting every calculation.

But distinguishing between just **two** states — "tube conducting" vs "tube not conducting," roughly 0V vs 5V — was easy and reliable even with imprecise components. There was a huge margin for error. A voltage of 3.7V or 4.2V could both be confidently read as "high" (1).

Before vacuum tubes, even earlier machines used **electromechanical relays** — physical switches flipped by electromagnets. A relay is literally either open or closed, on or off. Binary is the natural language of a switch.

When **transistors** replaced vacuum tubes in the late 1950s (and then **integrated circuits** packed thousands of transistors onto a single chip in the 1960s), they were far smaller and more reliable — but the binary principle remained, because the entire theoretical framework (Boolean algebra, logic gates, information theory) had already been built around two states. Today, a modern CPU contains **billions** of transistors, each one the size of a few nanometers, but each still operates as a binary switch — on or off, 0 or 1.

> The irony: modern transistors *could* reliably distinguish multiple voltage levels (and some technologies like **multi-level cell flash memory** in SSDs do exactly this, storing 2–4 bits per cell). But the binary abstraction is so deeply embedded in computing architecture — from hardware design to programming languages to network protocols — that switching away would mean redesigning everything from scratch.

### What Is a Byte?

A **byte** is a group of **8 bits** — eight tiny switches. A byte is the standard "unit" that computers work with. When people say "this file is 5 kilobytes," they mean it contains 5,000 groups of 8 bits (approximately — more on that later).

```
One byte = 8 bits (8 switches):

                      ← read this way (right to left)
  0  1  1  0  1  0  0  1
  ↑                    ↑
  bit 7 (most          bit 0 (least
  significant)         significant)
```

Notice that bits are **numbered from right to left** — bit 0 is on the far right, bit 7 is on the far left. This feels backwards at first, but it's the same convention you already know from decimal: in the number "142," the ones place is on the right, the hundreds place is on the left. We read the digits left to right, but the *positions* count up from right to left (ones → tens → hundreds). Binary follows the exact same rule: position 0 (worth 2⁰ = 1) is on the right, position 7 (worth 2⁷ = 128) is on the left.

### What Does "Significant" Mean?

The word **significant** here means **how much impact that bit has on the total value** — not "importance" in a general sense, but specifically **how much the number changes** if you flip that one bit.

Think about decimal first. In the number **142**:

```
  1     4     2
  ↑     ↑     ↑
  hundreds    tens     ones
  (most               (least
  significant)        significant)
```

- Changing the **ones** digit (2 → 3) changes the number by just **1** (142 → 143). Small impact.
- Changing the **hundreds** digit (1 → 2) changes the number by **100** (142 → 242). Huge impact.

The hundreds digit is the **most significant** digit because flipping it causes the **largest** change. The ones digit is the **least significant** because flipping it causes the **smallest** change. "Most" and "least" are relative to **each other** — they describe which position in the number has the greatest vs smallest influence on the total value.

Binary works exactly the same way, but with powers of 2 instead of powers of 10:

```
  0     1     1     0     1     0     0     1
  ↑                                         ↑
  bit 7                                     bit 0
  worth 128                                 worth 1
  (most significant bit — MSB)              (least significant bit — LSB)
```

- Flipping **bit 0** (the LSB) changes the value by **1**. Tiny impact.
- Flipping **bit 7** (the MSB) changes the value by **128**. Massive impact.

So when you see "most significant bit" (MSB) and "least significant bit" (LSB) in this documentation, it simply means the bit with the **biggest** influence on the number vs the bit with the **smallest** influence. In a byte, those are bit 7 (worth 128) and bit 0 (worth 1), respectively.

With 8 bits, you can represent **256 different values** (2⁸ = 256): from `00000000` (all switches off = 0) to `11111111` (all switches on = 255).

### Why Does This Matter?

In most high-level languages, you write `x = 42` and the language runtime decides how to store it (often as a 64-bit floating-point number — 8 whole bytes). You never think about this.

In low-level programming, you choose exactly how many bits to use:

| Rust Type | Size              | Range                    |
| --------- | ----------------- | ------------------------ |
| `u8`      | 1 byte (8 bits)   | 0 to 255                 |
| `u16`     | 2 bytes (16 bits) | 0 to 65,535              |
| `u32`     | 4 bytes (32 bits) | 0 to ~4 billion          |
| `i32`     | 4 bytes (32 bits) | -2 billion to +2 billion |
| `f64`     | 8 bytes (64 bits) | ±1.7 × 10³⁰⁸             |

Why does this matter? Because when you only have 1 MB of memory for your entire game, using 8 bytes for a number that only needs to be 0–3 is **wasteful**. You'd use a `u8` (1 byte), or even pack multiple values into a single byte (which is what bit-packing is — we'll get there).

---

## 2. Binary and Hexadecimal — Counting Differently

### Decimal (Base 10) — What You Know

You count in **base 10** every day. There are 10 digits (0–9), and each position is worth 10× more than the one to its right:

```
  1  4  2
  ↑  ↑  ↑
  │  │  └── 2 × 1     =   2
  │  └───── 4 × 10    =  40
  └──────── 1 × 100   = 100
                        ─────
                         142
```

### Binary (Base 2) — How Computers Count

Binary uses only 2 digits (0 and 1). Each position is worth 2× more than the one to its right:

```
  1  0  0  0  1  1  1  0
  ↑  ↑  ↑  ↑  ↑  ↑  ↑  ↑
  │  │  │  │  │  │  │  └── 0 × 1   =   0
  │  │  │  │  │  │  └───── 1 × 2   =   2
  │  │  │  │  │  └──────── 1 × 4   =   4
  │  │  │  │  └─────────── 1 × 8   =   8
  │  │  │  └────────────── 0 × 16  =   0
  │  │  └───────────────── 0 × 32  =   0
  │  └──────────────────── 0 × 64  =   0
  └─────────────────────── 1 × 128 = 128
                                     ─────
                                      142
```

So `10001110` in binary = `142` in decimal.

In Rust (and many languages), binary values are written with a `0b` prefix: `0b10001110`.

### Hexadecimal (Base 16) — A Shorthand for Binary

Binary is verbose. The number 255 is `11111111` — eight digits for a small number. **Hexadecimal** (hex, base 16) is a compact way to write binary. It uses 16 digits: `0-9` plus `A-F`:

| Decimal | Binary   | Hex |
| ------- | -------- | --- |
| 0       | 0000     | 0   |
| 1       | 0001     | 1   |
| 9       | 1001     | 9   |
| 10      | 1010     | A   |
| 15      | 1111     | F   |
| 255     | 11111111 | FF  |

Each hex digit represents exactly 4 bits. So one byte (8 bits) is always exactly 2 hex digits:

```
Binary:  1000 1110
Hex:       8    E    →  0x8E
Decimal:              →  142
```

You've seen hex before in CSS: `color: #FF00FF` means Red=FF (255), Green=00 (0), Blue=FF (255) — that's magenta. Each pair of hex digits is one byte.

---

## 3. Memory — A Giant Numbered Grid

### What Is RAM?

**RAM** (Random Access Memory) is your computer's working memory. Think of it as a massive array of bytes, each with a numbered address:

```
Address:  [0000]  [0001]  [0002]  [0003]  [0004]  [0005]  ...
Value:      72     101     108     108     111      33     ...
            'H'    'e'     'l'     'l'     'o'     '!'
```

When your program stores a variable, it's placing bytes at specific addresses in this array. When it reads a variable, it's looking up bytes by address.

### Why Does This Matter?

In high-level languages, memory is completely hidden from you. You create a variable and the runtime decides where to put it. You never know or care about addresses.

In low-level programming, you deal with memory directly:

```rust
// This string occupies 6 consecutive bytes starting at some address
// Each character is exactly 1 byte (in ASCII)
// The address is known, the layout is known, the size is known
```

### Addresses and Pointers

Every byte in memory has an **address** — a number that identifies its location. A **pointer** is a variable that holds an address. In most high-level languages, object references are secretly pointers — you just never see them.

In Rust, pointers are explicit. You know when you're borrowing a reference vs. copying data.

---

## 4. The Stack, the Heap, and Static Memory

In most high-level languages, memory management is invisible. The runtime allocates and garbage-collects for you. In low-level languages, you need to understand **where** data lives, because each location has different rules.

### The Stack

The **stack** is a region of memory used for local variables and function calls. It operates like a stack of plates — last in, first out (LIFO):

```
function a() {
    let x = 1;         // x is pushed onto the stack
    b();               // b's variables go on top of x
}                      // x is automatically removed when a() returns

function b() {
    let y = 2;         // y is pushed on top
}                      // y is automatically removed when b() returns
```

```
Stack growth (downward):

│             │
├─────────────┤
│  x = 1      │  ← a()'s local variable
├─────────────┤
│  y = 2      │  ← b()'s local variable (on top while b is running)
├─────────────┤
│             │
```

**Key properties:**
- ✅ **Extremely fast** — allocation is just moving a pointer
- ✅ **Automatic cleanup** — variables disappear when the function returns
- ❌ **Fixed size** — typically 1–8 MB. Put too much data here and you get a **stack overflow**
- ❌ **Short-lived** — data can't outlive the function that created it

### The Heap

The **heap** is a large pool of memory for data that needs to live longer or be larger than the stack allows. In garbage-collected languages, almost everything lives on the heap — objects, arrays, strings, closures. The garbage collector later figures out when to free it.

**Key properties:**
- ✅ **Flexible size** — can grow and shrink
- ✅ **Long-lived** — data persists until explicitly freed (or garbage collected)
- ❌ **Slower to allocate** — the allocator must find a free block
- ❌ **Fragmentation** — after many alloc/free cycles, free memory becomes scattered into small unusable gaps (see the [Memory Arena doc](02-memory-arena.md))
- ❌ **Requires cleanup** — in C you call `free()`, in Rust the compiler tracks ownership, in JS the garbage collector handles it

### Static Memory (The Data Segment)

**Static memory** is baked into the program's binary file. It exists before `main()` runs and lasts until the program exits. This is where `const` and `static` values live:

```rust
static GAME_TITLE: &str = "bit-bound";  // Embedded in the binary, exists forever
```

**Key properties:**
- ✅ **Zero allocation cost** — it's already there when the program starts
- ✅ **Lives forever** — never freed during program execution
- ❌ **Fixed at compile time** — can't grow or change size
- ❌ **Increases binary size** — every static byte makes the .exe larger

### How High-Level Languages Compare

| Data                    | High-Level Language                               | Low-Level (Rust)                                             |
| ----------------------- | ------------------------------------------------- | ------------------------------------------------------------ |
| A number `42`           | Runtime decides (usually heap or stack-optimized) | Stack (if `i32`)                                             |
| An array `[1,2,3]`      | Heap (runtime manages it)                         | Heap (if `Vec`), or Stack/Static (if fixed array `[i32; 3]`) |
| A constant `PI = 3.14`  | Runtime decides                                   | Static memory (compiled into binary)                         |
| A class/struct instance | Heap (garbage collected)                          | Heap (if `Box`), Stack (if small enough), Static (if global) |

In bit-bound, **the heap is completely forbidden**. Everything is either stack or static. This is the entire point of the arena allocator — it provides heap-like flexibility using a pre-allocated static block. See [Memory Arena](02-memory-arena.md) for how this works.

---

## 5. What Is a Pixel?

A **pixel** (short for **pic**ture **el**ement) is the smallest individual dot on a screen. Your monitor is a grid of pixels. If your monitor's resolution is 1920×1080, that means it has 1,920 columns and 1,080 rows of pixels — a total of 2,073,600 tiny colored dots.

Each pixel is a single point of color. When you zoom in far enough on any image, you can see individual pixels:

```
Normal view:      Zoomed in:

  🟦🟦🟦         ┌──┬──┬──┐
  🟦⬜🟦         │🟦│🟦│🟦│
  🟦🟦🟦         ├──┼──┼──┤
                  │🟦│⬜│🟦│  ← Each square is one pixel
                  ├──┼──┼──┤
                  │🟦│🟦│🟦│
                  └──┴──┴──┘
                  3×3 = 9 pixels
```

In most programming, you work with pixels indirectly — CSS handles layout, UI frameworks handle drawing, canvas APIs abstract the details. You never think about *how* a pixel is stored in memory.

In low-level graphics, a pixel is just a number (or set of numbers) stored at a specific position in a memory array. See [FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md) for how bit-bound stores pixels.

---

## 6. How Screens Display Images

### The Refresh Cycle

Your screen doesn't show a single frozen image. It **redraws** the entire screen many times per second:

1. The graphics system writes pixel data into a memory region (the **framebuffer** — more on this later)
2. The display hardware reads that memory region and converts each pixel's value into light
3. This happens 60 times per second (on a 60 Hz display), 144 times per second (on a 144 Hz display), etc.

Each complete redraw is called a **frame**. At 60 Hz, each frame lasts 16.67 milliseconds.

Imagine a flipbook — a stack of paper with slightly different drawings on each page. When you flip through them quickly, the drawings appear to move. A screen works the same way:

- Each "page" is a frame
- Each frame is a complete picture made of thousands/millions of pixels
- Flipping 60 pages per second creates the illusion of motion

But *why* does flipping static images create the illusion of motion? Because of how your eyes and brain actually work.

### How Your Eyes See Motion

Your eyes don't see the world as a continuous video stream. Instead, your retina captures **discrete snapshots** — light hits the photoreceptor cells, they fire signals to the brain, and then they need a brief recovery period before they can fire again. Your brain receives these individual snapshots and **blends them together**, filling in the gaps to create the perception of smooth, continuous motion. This is called **persistence of vision** — each image lingers in your visual system for a fraction of a second, overlapping with the next one.

A flipbook exploits this: if you replace one still image with a slightly different one fast enough (roughly 12+ times per second), your brain can't distinguish the individual images anymore — it perceives movement instead. Film runs at 24 frames per second. Most screens refresh at 60 Hz. Both are fast enough to completely fool the brain.

A computer screen does the exact same thing — it displays one complete image (a frame), then replaces it with a slightly different one, 60 times per second. The computer's job is to **produce a new frame fast enough** for the screen to display it. If the computer can't keep up, your brain *can* tell — you perceive it as stuttering or lag, because the gaps between frames become long enough for your visual system to notice.

---

## 7. How Computers Represent Colors

### Modern Color: 32-bit RGBA

Most modern systems represent color with four channels: Red, Green, Blue, and Alpha (transparency). Each channel gets 8 bits (0–255), for a total of 32 bits (4 bytes) per pixel. You may have seen this in CSS (`#FF0000` = pure red, `rgb(255, 0, 0)`), image editors, or graphics libraries. This scheme gives 16.7 million possible colors, but each pixel costs **4 bytes** of memory.

### The Retro Way: What bit-bound Uses

Old hardware couldn't afford 4 bytes per pixel. The original Game Boy had 4 shades of gray-green, each pixel needing only **2 bits**:

```
Value   Binary    Color
  0      00       Lightest  ░░
  1      01       Light     ▒▒
  2      10       Dark      ▓▓
  3      11       Darkest   ██
```

That's just 4 possible values. With 2 bits per pixel instead of 32, the memory savings are enormous:

```
160 × 144 pixels = 23,040 pixels

32-bit color:  23,040 × 4 bytes = 92,160 bytes (90 KB)
2-bit color:   23,040 × 2 bits  = 46,080 bits  = 5,760 bytes (< 6 KB)
```

The 2-bit version uses **16× less memory**. When your total RAM is 1 MB, this matters a lot.

### Palettes

A **palette** is a lookup table that maps small index numbers to actual colors. Instead of storing "this pixel is RGB(155,188,15)" (3 bytes), you store "this pixel is color index 1" (2 bits), and separately define that "index 1 = RGB(155,188,15)."

```
Palette (defined once):
  Index 0 → RGB(155, 188, 15)   lightest green
  Index 1 → RGB(139, 172, 15)   light green
  Index 2 → RGB(48, 98, 48)     dark green
  Index 3 → RGB(15, 56, 15)     darkest green

Pixel data (per pixel, just 2 bits):
  0, 0, 3, 3, 3, 3, 0, 0
  0, 3, 1, 1, 1, 1, 3, 0
  ...
```

When it's time to display the image on a modern screen, each 2-bit index is looked up in the palette to get the actual RGB color. This conversion happens at display time, not at storage time.

---

## 8. What Is a Buffer?

### General Concept

A **buffer** is a temporary holding area for data. It exists because **a producer and a consumer often operate at different speeds**, and you need somewhere to put the data in between.

Think of a YouTube video buffering:
1. Your internet downloads video data (the **producer**)
2. Your video player displays frames (the **consumer**)
3. The downloaded-but-not-yet-displayed data sits in a **buffer**

Without the buffer, the video would stutter every time the download hiccupped. The buffer absorbs the speed difference.

### In Programming

You've used buffers in JavaScript without knowing it:

```javascript
// Node.js Buffer — a chunk of raw binary data
const buf = Buffer.from("Hello");  // [72, 101, 108, 108, 111]

// Streams use internal buffers
const stream = fs.createReadStream("bigfile.txt");
// Data is read into a buffer, then emitted in chunks
```

A buffer is just **a block of memory used to temporarily hold data** between two operations.

---

## 9. What Is a Frame?

A **frame** is one complete image that the screen displays. At 60 frames per second (FPS), the screen shows 60 complete images every second, each lasting ~16.67 milliseconds.

```
Time:  0ms        16.6ms      33.3ms      50ms        66.6ms
       │           │           │           │           │
       ├─ Frame 1  ┤─ Frame 2  ┤─ Frame 3  ┤─ Frame 4  ┤
       │  (draw    │  (draw    │  (draw    │  (draw    │
       │  screen)  │  screen)  │  screen)  │  screen)  │
```

The word "frame" comes from film/animation: each photograph in a film strip is a "frame." Computer graphics borrowed the term.

### Frame vs. Tick

In bit-bound, a "frame" and a "tick" are the same thing because we use a **fixed timestep** (see [Fixed Timestep](06-fixed-timestep.md)). But in more complex engines, they can differ — game logic might tick 30 times per second while rendering happens 144 times per second with interpolation in between.

---

## 10. What Is a Framebuffer?

Now we can combine the two concepts:

- A **buffer** is a temporary holding area for data
- A **frame** is one complete screen image

A **framebuffer** is a region of memory that holds the pixel data for one complete frame. It's the buffer between your game logic ("I want this pixel to be blue") and the screen hardware ("I need to display something right now").

```
Your Game Logic                Framebuffer               Screen
──────────────          ─────────────────────          ──────────
"Draw player                 [Memory block             Your eyes
 at (10, 20)"   ──────►    with pixel data]  ──────►    see the
"Draw enemy                  Row by row,                image
 at (50, 70)"               byte by byte
```

### Why "Buffer" and Not Just "Image"?

Because the framebuffer is a **temporary workspace**. Each frame, your game:
1. **Clears** the framebuffer (sets all pixels to the background color)
2. **Writes** new pixel data (draws the current game state)
3. **Presents** the framebuffer to the screen

Then next frame, it starts over. The framebuffer is constantly being overwritten — it's not an archive, it's a scratchpad.

### In bit-bound

The framebuffer is a flat array of 5,760 bytes representing 23,040 pixels (160×144), with each byte holding 4 pixels (2 bits each). For the full technical details, see [FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md).

---

## 11. What Is Rendering?

**Rendering** is the process of producing a visual image from data. It's the act of converting abstract game state ("the player is at position 50, 80") into concrete pixels in the framebuffer.

### On The Web

The browser renders for you:

```javascript
document.body.innerHTML = "<h1>Hello</h1>";
// The browser's renderer:
// 1. Parses the HTML into a DOM tree
// 2. Calculates layout (where does this h1 go?)
// 3. Paints pixels onto the screen
// You never think about individual pixels.
```

### In Low-Level Graphics

In bit-bound, **you are the renderer**. There is no browser, no CSS engine, no layout engine. If you want a character on screen, you write the pixel values yourself:

```rust
// To draw a 3-pixel wide "L" shape:
framebuffer.set_pixel(x, y,     3);      // █
framebuffer.set_pixel(x, y + 1, 3);      // █
framebuffer.set_pixel(x, y + 2, 3);      // █
framebuffer.set_pixel(x + 1, y + 2, 3);  // ██
framebuffer.set_pixel(x + 2, y + 2, 3);  // ███
```

Every frame, your rendering code must:
1. Clear the screen
2. Draw the background
3. Draw the entities (enemies, items)
4. Draw the player
5. Draw the UI (health bar, score)

This happens 60 times per second. If it takes longer than 16.6ms, the game slows down.

---

## 12. What Is a Tick?

A **tick** is one complete cycle of your game loop — all the work the game does to advance by one "step." In bit-bound, one tick = one frame = 1/60th of a second.

Each tick does the following, in order:

```
┌─────────────── One Tick (16.67ms budget) ────────────────┐
│                                                          │
│  1. Reset the frame arena (clear temporary memory)       │
│  2. Read input (what buttons is the player pressing?)    │
│  3. Update game state (move entities, check collisions)  │
│  4. Render (draw everything to the framebuffer)          │
│  5. Present (send the framebuffer to the screen)         │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Why "Tick"?

The term comes from the "tick" of a clock — a regular, rhythmic beat. Each tick, the game world advances by one fixed unit of time. Like a clock ticks exactly once per second, the game ticks exactly 60 times per second.

In JavaScript, the closest equivalent is `requestAnimationFrame`:

```javascript
function gameLoop() {
    update();      // ← one "tick"
    render();
    requestAnimationFrame(gameLoop);  // schedule next tick
}
```

But `requestAnimationFrame` fires at the monitor's refresh rate (variable), whereas bit-bound's tick is a strict, unwavering 60 FPS. See [Fixed Timestep](06-fixed-timestep.md) for why this matters.

---

## 13. What Is a Sprite?

### The Simple Answer

A **sprite** is a small 2D image — like a character, a bullet, or a treasure chest — that gets drawn onto the screen at a specific position. The word "sprite" literally means "a small fairy or elf" — early computer graphics engineers at Texas Instruments named them after the mythical creatures because these little images seemed to "float" independently over the background.

### How a Sprite Is Stored

In web development, a sprite is an image file (PNG, SVG). In low-level programming, a sprite is just **an array of numbers** where each number represents a pixel's color:

```
What you see:              How it's stored:

    ██████                 0,0,3,3,3,3,0,0,
  ██░░░░░░██               0,3,1,1,1,1,3,0,
██░░▓▓░░▓▓░░██             3,1,2,1,1,2,1,3,
██░░░░░░░░░░██             3,1,1,1,1,1,1,3,
██░░░░▓▓▓▓░░██             3,1,1,2,2,1,1,3,
██░░░░░░░░░░██             3,1,1,1,1,1,1,3,
  ██░░░░░░██               0,3,1,1,1,1,3,0,
    ██████                 0,0,3,3,3,3,0,0
```

- `0` = transparent (not drawn — shows whatever is behind the sprite)
- `1` = lightest color
- `2` = dark color
- `3` = darkest color

This array is stored in **row-major order** — the first 8 numbers are the first row, the next 8 are the second row, and so on.

### In Code

```rust
// A sprite is a compile-time constant — just bytes.
const FACE: [u8; 64] = [
    0,0,3,3,3,3,0,0,
    0,3,1,1,1,1,3,0,
    3,1,2,1,1,2,1,3,  // the 2s are "eyes"
    3,1,1,1,1,1,1,3,
    3,1,1,2,2,1,1,3,  // the 2s are a "mouth"
    3,1,1,1,1,1,1,3,
    0,3,1,1,1,1,3,0,
    0,0,3,3,3,3,0,0,
];
const FACE_WIDTH: usize = 8;
const FACE_HEIGHT: usize = 8;
```

### How It Gets Drawn

The sprite renderer iterates over every pixel in the array and copies it to the framebuffer at the desired screen position, skipping transparent (0) pixels:

```
For each pixel in the sprite:
  1. Read the color value from the array
  2. If color == 0, skip (transparent)
  3. Calculate where this pixel should go on screen:
     screen_x = sprite_x + column
     screen_y = sprite_y + row
  4. If the screen position is off-screen, skip (clipping)
  5. Write the color to the framebuffer at that position
```

This is the fundamental operation of all 2D game rendering — stamp small images onto a big image, 60 times per second.

For the full technical details, see [Sprite Rendering](05-sprite-rendering.md).

---

## 14. Bitwise Operations — The Programmer's Screwdriver

In JavaScript, you almost never use bitwise operations. In low-level programming, they're everywhere — especially for bit-packing (storing multiple values in one byte). Here are the operations you'll encounter:

### AND (`&`) — "Keep only the bits where both are 1"

```
  10110110
& 00001111
──────────
  00000110    (keeps the lower 4 bits, zeros the upper 4)
```

Used for **masking** — extracting specific bits from a value.

### OR (`|`) — "Set bits where either is 1"

```
  10110000
| 00000110
──────────
  10110110    (combines two values without disturbing each other's bits)
```

Used for **combining** — writing bits into specific positions.

### NOT (`!` or `~`) — "Flip every bit"

```
~ 00001111
──────────
  11110000    (turns 0s into 1s and vice versa)
```

Used to create **inverse masks**.

### Shift Left (`<<`) — "Move all bits to the left by N positions"

```
  00000011 << 4
──────────────
  00110000      (the value 3 moved into bits 4–5)
```

Used for **positioning** — putting a value at a specific bit offset.

### Shift Right (`>>`) — "Move all bits to the right by N positions"

```
  00110000 >> 4
──────────────
  00000011      (extracts the value from bits 4–5)
```

Used for **extracting** — reading a value from a specific bit offset.

### Why This Matters for bit-bound

When you pack 4 pixels into 1 byte (2 bits per pixel), you use these operations constantly:

```rust
// Writing pixel 2 (color = 3) into a byte:
//   bit_offset = 2 * 2 = 4
//   Clear the slot:  byte &= !(0b11 << 4)    → clears bits 4–5
//   Write the color: byte |= (3 << 4)         → sets bits 4–5 to 11

// Reading pixel 2 from a byte:
//   color = (byte >> 4) & 0b11                → shifts bits 4–5 down, masks
```

This is covered in detail in [FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md).

---

## What's Next?

Now that you understand the fundamentals, read the concept docs in this order:

1. **[Hardware-Constrained Design](01-hardware-constrained-design.md)** — Understand the "why" behind all the constraints
2. **[Memory Arena](02-memory-arena.md)** — How memory management works without a heap
3. **[FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md)** — How pixel data is stored in memory
4. **[Sprite Rendering](05-sprite-rendering.md)** — How images are drawn onto the framebuffer
5. **[Fixed-Capacity Entities](07-fixed-capacity-entities.md)** — How game objects are managed
6. **[Fixed Timestep](06-fixed-timestep.md)** — How the game loop maintains consistent timing
7. **[Static Memory Patterns](03-static-memory-patterns.md)** — Where data lives in the binary
8. **[Debug Overlays](08-debug-overlays.md)** — How development tools are built with the same constraints
