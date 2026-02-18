# Hardware-Constrained Design

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially sections on memory, pixels, and colors. This document provides the philosophical "why" behind the constraints that every other concept doc implements.
>
> **Related ADR**: [0004 — Hardware Constraints](../adr/0004-hardware-constraints.md)

---

## What Is Hardware-Constrained Design?

**Hardware-constrained design** is the practice of voluntarily imposing strict technical limitations — on resolution, color depth, memory, processing power — and then building within those limits. The constraints are typically inspired by real historical hardware (like the Game Boy or NES) but enforced purely through software, not actual hardware.

bit-bound imposes these constraints:

| Constraint        | Value                       | Inspiration                        |
| ----------------- | --------------------------- | ---------------------------------- |
| Screen resolution | 160 × 144 pixels            | Game Boy (1989)                    |
| Color depth       | 4 colors (2 bits per pixel) | Game Boy's 4 shades                |
| Framebuffer size  | 5,760 bytes                 | Derived: 160×144÷4 pixels per byte |
| Total memory      | 1 MB                        | Self-imposed budget                |
| Tile size         | 16 × 16 pixels              | Common retro tile standard         |
| Frame rate        | 60 FPS fixed                | NTSC standard                      |
| Max enemies       | 32                          | Self-imposed budget                |
| Max projectiles   | 64                          | Self-imposed budget                |
| Heap allocation   | Forbidden                   | Design philosophy                  |

None of these are limits of the developer's actual computer. A modern PC has gigabytes of RAM and can render millions of pixels. These constraints are **chosen on purpose** as a creative and technical exercise.

---

## Why Voluntarily Limit Yourself?

This might seem counterintuitive. Why would you deliberately make things harder? Because constraints do something powerful: they **eliminate choices** and **force creativity**.

### Problem 1: Decision Paralysis

Without constraints, every decision is open-ended:
- What resolution? Could be anything from 320×240 to 3840×2160
- How many colors? 16? 256? 16 million? True HDR?
- Art style? Pixel art? Vector? 3D? Hand-painted?
- How much memory? As much as you want

With constraints, most of these decisions are already made:
- Resolution: 160×144. Done.
- Colors: 4. Done.
- Art style: 4-color pixel art. It's the only option.
- Memory: 1 MB. Plan accordingly.

This dramatically speeds up development. You spend time **building**, not deciding.

### Problem 2: Scope Creep

Without limits, a side project can expand forever — "let's add particle effects," "let's add HD textures," "let's add multiplayer." With hard limits, scope is naturally bounded:

- You can't build a particle system with 10,000 particles when you have 64 projectile slots
- You can't add HD sprites when your entire screen is 160×144
- You can't add a complex physics engine when you have 16.67ms per tick and a 1 MB memory budget

The constraints act as **automatic scope control**.

### Problem 3: Architectural Sloppiness

When resources feel infinite (modern computers have 16+ GB of RAM), developers tend to use them carelessly:

```
// In an unconstrained environment, this "just works":
let enemies = Vec::new();            // Dynamic, unbounded
let particles = Vec::new();          // Dynamic, unbounded
let tiles = HashMap::new();          // Hash table with overhead
let player_name = String::from(...); // Heap-allocated string
```

When memory is 1 MB, **every allocation is visible and intentional**:

```
// In bit-bound, every byte is accounted for:
let enemies = arena.alloc_slice::<Enemy>(32);       // 32 enemies, known cost
let framebuffer = [0u8; 5_760];                      // exact pixel budget
// No Vec, no HashMap, no String — they're forbidden
```

This forces explicit reasoning about data structures, lifetimes, and memory layout — skills that transfer to any performance-critical codebase. See [Memory Arena](02-memory-arena.md) for how these constraints shape the allocation system.

### Problem 4: Art Inconsistency

When there are no constraints, different parts of a project can look inconsistent — one artist makes high-fidelity sprites, another uses placeholder cubes, a programmer throws in debug rectangles. The art has no unifying style.

When everyone works within the same 4-color, 16×16 pixel tile grid, all assets naturally share a consistent aesthetic. The constraint **is** the style guide.

---

## How Constraints Are Enforced

The constraints aren't just written in a document — they're enforced in code. Each constraint is a technical decision that makes violations impossible or immediately obvious:

| Constraint         | Enforcement                                                                    | What Happens If Violated                                  |
| ------------------ | ------------------------------------------------------------------------------ | --------------------------------------------------------- |
| 160×144 resolution | [FrameBuffer](04-framebuffer-bit-packing.md) struct has exactly 5,760 bytes       | Can't address pixels outside this range                   |
| 4 colors           | [set_pixel](04-framebuffer-bit-packing.md) masks color to 2 bits (`color & 0b11`) | Values > 3 are silently clamped                           |
| 1 MB memory        | [Arena allocator](02-memory-arena.md) returns error when full                     | Game receives `MemoryError`, can't silently exceed budget |
| 32 enemies         | [Fixed array](07-fixed-capacity-entities.md) of size 32                           | `spawn_enemy` returns `None` when pool is full            |
| No heap allocation | `Vec`, `Box`, `String` not used in core code                                   | Compile-time discipline (no automatic enforcement)        |
| 60 FPS fixed       | [Game loop](06-fixed-timestep.md) sleeps to maintain cadence                      | Game slows down rather than skipping frames               |

### The Constraint Budget as Architecture

These constraints aren't isolated rules — they form an interconnected system where each constraint shapes the others:

```
Resolution (160×144) + Colors (4)
  → FrameBuffer size (5,760 bytes)      — see FrameBuffer & Bit-Packing
  → Bit-packing required (2 bpp)         — see FrameBuffer & Bit-Packing

Total Memory (1 MB) + No Heap
  → Arena allocator required              — see Memory Arena
  → Static memory placement               — see Static Memory Patterns
  → Fixed-capacity entity arrays           — see Fixed-Capacity Entities

60 FPS Fixed
  → 16.67ms per-tick budget               — see Fixed Timestep
  → Frame Arena reset each tick            — see Memory Arena
  → Debug overlay for monitoring           — see Debug Overlays
```

Every concept doc in this folder exists because of a constraint defined here.

---

## The Inspiration: Real Hardware Constraints

bit-bound's constraints aren't arbitrary — they're modeled after real hardware that shaped the history of game development.

### Game Boy (1989) — The Primary Inspiration

| Spec       | Game Boy                 | bit-bound                                |
| ---------- | ------------------------ | ---------------------------------------- |
| Resolution | 160 × 144                | 160 × 144 (identical)                    |
| Colors     | 4 shades of green        | 4 colors (palette-mapped)                |
| CPU        | Sharp LR35902 @ 4.19 MHz | Modern CPU (self-limited by design)      |
| Work RAM   | 8 KB                     | 1 MB (more generous)                     |
| VRAM       | 8 KB                     | 5,760 bytes framebuffer                  |
| Sprites    | 40 (8×8 or 8×16)         | Unlimited size, limited count (32+64+16) |

The Game Boy sold 118 million units and produced thousands of beloved games — all within these brutal limits. Games like *Pokémon*, *The Legend of Zelda: Link's Awakening*, and *Tetris* are masterpieces of constrained design.

### NES (1983)

- 2 KB work RAM, 2 KB video RAM
- 256 tiles in pattern memory (background + sprites shared a pixel budget)
- 64 sprites, 8 per scanline (exceeding this caused **flickering**)
- 4 palettes of 4 colors each

*Super Mario Bros.* famously reused cloud sprites as bush sprites (just different palette colors) because tiles were scarce. Constraint → creativity.

### Atari 2600 (1977)

- **128 bytes** of RAM. Not kilobytes — **bytes**.
- No framebuffer at all — the CPU generated each scanline in real time ("racing the beam")
- 2 player sprites, 2 missile sprites, 1 ball sprite, 1 playfield — that's everything

Despite having less memory than a single text message, the 2600 produced over 500 commercial games.

---

## Designing Within Constraints: A Mindset Comparison

| Decision                        | Without Constraints                                       | With bit-bound's Constraints                                         |
| ------------------------------- | --------------------------------------------------------- | -------------------------------------------------------------------- |
| "How do I show enemies?"        | Load animated sprite sheets, GPU-rendered, any resolution | Hand-author `const` byte arrays, 4 colors, 16×16 max                 |
| "How do I handle memory?"       | `Vec`, `HashMap`, garbage collector                       | [Arena allocator](02-memory-arena.md), pre-sized, no heap               |
| "How do I spawn entities?"      | `enemies.push(Enemy::new())`                              | Find an inactive slot in a [fixed array](07-fixed-capacity-entities.md) |
| "How do I handle transparency?" | Alpha blending (0–255 opacity per pixel)                  | [Color 0 = transparent](05-sprite-rendering.md)                         |
| "How do I display debug info?"  | `console.log()` or ImGui                                  | [3×5 bitmap font](08-debug-overlays.md) drawn to framebuffer            |
| "How do I time the game?"       | `requestAnimationFrame` / variable `dt`                   | [Fixed 60 FPS timestep](06-fixed-timestep.md), no `dt`                  |

---

## History

### Constraints Born of Necessity

- **1972 — Pong**: Built entirely in discrete hardware logic (NAND gates, flip-flops). No CPU, no software. Every visual element existed because a specific wire carried a specific signal. There were no constraints to choose — there were only constraints.
- **1977 — Atari 2600**: 128 bytes of RAM, no framebuffer. Developers invented "racing the beam" — generating each scanline in the exact number of CPU cycles before the electron beam reached that line. Extraordinary games emerged from extraordinary limitations.
- **1983 — NES**: 2 KB RAM. Super Mario Bros. (40 KB cartridge) defined an entire genre. Cloud = bush sprites (recolored). Constraint → creativity.
- **1989 — Game Boy**: 160×144, 4 shades of green. 118 million units sold. Proof that constraints don't limit success — they focus it.
- **1993 — Doom**: Software-rendered on a 386 with no 3D hardware. id Software invented BSP trees, fixed-point math, and height-field rendering to create a "3D" game. The constraint (no GPU) forced innovations that influenced 30 years of game engines.

### Constraints as Intentional Choice

- **2000s — Demoscene**: The demoscene community creates programs with extreme self-imposed size limits (4 KB, 64 KB, sometimes 256 bytes). These "intros" produce astonishing visual output, proving that constraints breed innovation.
- **2015 — Pico-8**: Lexaloffle created a "fantasy console" — a virtual machine with strict, Game Boy-era limitations (128×128, 16 colors, 32 KB cartridge). It spawned a massive creative community. The constraints are the product.
- **2010s–present — Game jams**: Events like Ludum Dare and GMTK Jam frequently use constraint-based themes. The creative output consistently shows that tighter limits produce more focused, inventive designs.
- **bit-bound**: Follows this tradition — using Rust and self-imposed constraints as both an educational framework and a creative exercise.

## Alternatives

| Approach                              | Description                                                   | Trade-off                                                                               |
| ------------------------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| **Unconstrained modern development**  | Use full GPU, unlimited memory, 4K resolution.                | Maximum fidelity, but scope and complexity explode. No architectural forcing function.  |
| **Fantasy consoles** (Pico-8, TIC-80) | Pre-built virtual machines with fixed constraints.            | Excellent for quick prototyping. But you don't learn to *build* the constraint system.  |
| **Actual retro homebrew**             | Write code for real hardware (Game Boy, NES via assembly).    | Maximum authenticity, but requires hardware-specific assembly. Steeper learning curve.  |
| **Game jams**                         | Time-based constraints (48 hours) rather than technical ones. | Forces scope discipline, but doesn't teach low-level architecture.                      |
| **Progressive constraints**           | Start unconstrained, then optimize after the fact.            | More practical for production, but doesn't provide the same up-front design discipline. |

## Further Reading

- [Foundations — All Sections](00-foundations.md) — The building blocks that every constraint relies on
- [Rodrigo Copetti — Console Architecture Series](https://www.copetti.org/writings/consoles/) — Beautifully illustrated breakdowns of real console hardware
- [Pico-8 — Fantasy Console](https://www.lexaloffle.com/pico-8.php) — The most popular fantasy console
- ["Racing the Beam" by Montfort & Bogost](https://mitpress.mit.edu/9780262539760/racing-the-beam/) — Atari 2600 development under extreme constraints
- [Demoscene — Pouet.net](https://www.pouet.net) — Archive of size-constrained creative coding
