# Fixed Timestep Game Loop

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially sections on frames, ticks, and rendering.
>
> **Related ADR**: [0003 — Deterministic Fixed Timestep](../adr/0003-deterministic-timestep.md)

---

## What Is a Game Loop?

Before understanding "fixed timestep," we need to understand what a **game loop** is.

In most software, the program **reacts to events** — a user clicks a button, a server receives a request, a timer fires. The program does nothing until something happens. This is called **event-driven programming**, and it's how most web servers, GUIs, and mobile apps work.

A game is fundamentally different. A game must **constantly update and redraw** — even if the player isn't pressing any buttons, enemies are still moving, animations are still playing, and physics is still simulating. The game must proactively do work 60+ times per second, whether or not anything "happened."

This continuous cycle is called the **game loop**:

```
while game_is_running {
    read_input();       // What is the player doing?
    update_state();     // Move entities, check collisions, apply physics
    render();           // Draw everything to the framebuffer
    present();          // Show the framebuffer on screen
}
```

Every iteration of this loop produces one **frame** (one complete screen image) and advances the game world by one **tick** (one step of simulation). The loop runs continuously, as fast as it can — or, in bit-bound's case, at a locked 60 iterations per second.

---

## What Does "Timestep" Mean?

A **timestep** (or **time step**) is the amount of simulated time that passes with each iteration of the game loop. It answers the question: "How much does the game world advance each tick?"

There are two approaches:

### Variable Timestep

Each tick, you measure how much real time has passed since the last tick (the **delta time**, or `dt`), and use that to scale all calculations:

```rust
// Variable timestep — every calculation multiplied by dt
let dt = time_since_last_frame();  // could be 0.016s, could be 0.032s

position += velocity * dt;
velocity += gravity * dt;
cooldown -= dt;
```

If the previous frame took 16ms, `dt` = 0.016. If it took 33ms (because the CPU was busy), `dt` = 0.033. The game scales its calculations to compensate.

### Fixed Timestep

Each tick represents **exactly the same duration** — always. In bit-bound, every tick is exactly 1/60th of a second (≈16.67 milliseconds), no exceptions:

```rust
// Fixed timestep — no dt needed
position += velocity;     // velocity is "pixels per tick", not "per second"
velocity += gravity;      // gravity is "acceleration per tick"
cooldown -= 1;            // cooldown measured in ticks, not seconds
```

No multiplication by `dt`. No fractional time. Every tick is identical.

---

## Why Does bit-bound Use a Fixed Timestep?

### Problem 1: Variable Timestep Breaks Determinism

**Determinism** means "the same inputs always produce the same outputs." This is crucial for:
- **Replays**: Record the player's inputs and play them back to reproduce the exact same game
- **Debugging**: A bug that only happens on frame 4,721 can be reliably reproduced
- **Testing**: Automated tests produce identical results every run

With a variable timestep, the game behaves differently depending on how fast the computer runs. A character moving at the same "speed" will end up at slightly different positions depending on the framerate, because floating-point arithmetic is not perfectly linear:

```
At 60 FPS (dt = 0.01667):
  position after 60 frames = 60 × (1.0 × 0.01667) ≈ 1.0002

At 30 FPS (dt = 0.03333):
  position after 30 frames = 30 × (1.0 × 0.03333) ≈ 0.9999
```

The results are slightly different. Over thousands of frames, this divergence accumulates and becomes significant.

With a fixed timestep, both machines execute the exact same calculations with the exact same values, producing pixel-identical results.

### Problem 2: Physics Explodes at Extreme Framerates

Variable timestep can cause "physics explosions" — objects tunneling through walls, jumping higher on faster machines, or falling faster on slower ones. This happens because physics calculations are sensitive to the size of `dt`:

- **Very small dt** (very high framerate): Floating-point precision issues accumulate over many tiny increments
- **Very large dt** (lag spike): An object might move 100 pixels in a single step, passing through a wall that's only 10 pixels thick

Fixed timestep eliminates this entirely — every step moves the same amount.

### Problem 3: Code Complexity

With variable timestep, **every single time-dependent calculation** must be multiplied by `dt`. Miss one, and you have a bug that only appears at specific framerates:

```rust
// Variable — dt must be threaded through everything
position += velocity * dt;
velocity += gravity * dt;
animation_timer += dt;
invincibility_timer -= dt;
particle_lifetime -= dt;
spawn_cooldown -= dt;
// Forgot to multiply something by dt? Bug that depends on framerate.
```

```rust
// Fixed — clean, simple, no dt
position += velocity;
velocity += gravity;
animation_timer += 1;
invincibility_timer -= 1;
particle_lifetime -= 1;
spawn_cooldown -= 1;
// Every value is in "per tick" units. Can't forget dt — there is no dt.
```

---

## How bit-bound's Game Loop Works

### The Loop

```rust
const TICK_DURATION: Duration = Duration::from_nanos(16_666_667); // ≈1/60th of a second

loop {
    let tick_start = Instant::now();

    // ── One complete tick ──────────────────────────
    frame_arena.reset();        // Clear temporary memory (see Memory Arena doc)
    poll_input();               // Read controller state
    update_game_state();        // Move entities, check collisions
    render(&mut framebuffer);   // Draw to the framebuffer
    present(&framebuffer);      // Send framebuffer to screen
    // ───────────────────────────────────────────────

    // Wait until the tick duration has elapsed
    let elapsed = tick_start.elapsed();
    if elapsed < TICK_DURATION {
        sleep(TICK_DURATION - elapsed);
    }
    // If elapsed > TICK_DURATION, the game just runs slow (no skipping)
}
```

### What Happens Inside One Tick

```
┌─────────────────── One Tick (16.67ms budget) ───────────────────┐
│                                                                  │
│  1. Reset Frame Arena                                            │
│     └─ Clear all temporary per-frame memory. See Memory Arena.   │
│                                                                  │
│  2. Poll Input                                                   │
│     └─ Read the D-pad and buttons. Store as a bitmask.           │
│                                                                  │
│  3. Update Game State                                            │
│     ├─ Move the player based on input                            │
│     ├─ Move enemies according to their AI                        │
│     ├─ Move projectiles along their trajectories                 │
│     ├─ Check for collisions (player↔enemy, projectile↔enemy)     │
│     └─ Update timers, spawn new entities, etc.                   │
│                                                                  │
│  4. Render                                                       │
│     ├─ Clear the framebuffer (all pixels to background color)    │
│     ├─ Draw the tilemap / background                             │
│     ├─ Draw enemies (using draw_sprite — see Sprite Rendering)   │
│     ├─ Draw projectiles                                          │
│     ├─ Draw the player                                           │
│     └─ Draw UI (health, score) and debug overlay if enabled      │
│                                                                  │
│  5. Present                                                      │
│     └─ Send the framebuffer to the display output adapter        │
│                                                                  │
│  6. Sleep                                                        │
│     └─ Wait for remaining time until next tick                   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### What Happens When the Tick Takes Too Long?

If the game logic + rendering takes more than 16.67ms, the game **slows down**. It doesn't skip ticks or drop frames — it just runs at a lower-than-60 FPS until the load decreases.

This is the same behavior as classic consoles: when the NES had too many sprites on screen, the game visibly lagged. This is actually **desirable** for determinism:

- Every input is processed (nothing is skipped)
- No simulation steps are dropped
- The game is always in a consistent state

You can observe this by enabling the [Debug Overlay](08-debug-overlays.md), which shows the current FPS in real time.

---

## Measuring Time in Ticks, Not Seconds

In bit-bound, time is measured in ticks, not seconds or milliseconds. This is a subtle but important consequence of the fixed timestep:

| What You Want          | Variable Timestep (in seconds)    | Fixed Timestep (in ticks)        |
| ---------------------- | --------------------------------- | -------------------------------- |
| 2-second cooldown      | `cooldown = 2.0; cooldown -= dt;` | `cooldown = 120; cooldown -= 1;` |
| Walk speed             | `speed = 60.0` pixels/second      | `speed = 1` pixel/tick           |
| 3-second invincibility | `timer = 3.0; timer -= dt;`       | `timer = 180; timer -= 1;`       |

Since there are always exactly 60 ticks per second, converting is easy: **ticks = seconds × 60**. And since ticks are integers (not floating-point), there are no precision issues.

---

## Connection to Other Concepts

The fixed timestep loop is the backbone that ties everything together:

- **[Memory Arena](02-memory-arena.md)**: The Frame Arena is reset at the start of every tick — this is step 1 of the loop
- **[FrameBuffer & Bit-Packing](04-framebuffer-bit-packing.md)**: The framebuffer is cleared and redrawn every tick — this is step 4
- **[Sprite Rendering](05-sprite-rendering.md)**: Sprites are drawn to the framebuffer during the render step
- **[Fixed-Capacity Entities](07-fixed-capacity-entities.md)**: Entity arrays are iterated and updated during step 3
- **[Debug Overlays](08-debug-overlays.md)**: The FPS counter measures how many ticks complete per second

---

## History

- **1970s–80s — Arcade and console hardware**: There was no concept of "variable framerate." The CPU, display controller, and CRT monitor were all synchronized to the same clock. The Atari 2600 literally drew the screen one scanline at a time, with game logic interleaved between scanlines. The NES ran at exactly 60.0988 FPS (NTSC) or 50.007 FPS (PAL), locked to the TV's vertical blank interrupt. **Every game on these platforms was inherently fixed-timestep.**
- **1990s — PCs introduce variable framerates**: As PCs replaced consoles for gaming, developers faced wildly different hardware. A game might run at 15 FPS on one machine and 120 FPS on another. The naive solution — tying game speed to framerate — resulted in games that played faster on faster computers. Early PCs even had a "Turbo" button because of this.
- **1996 — Quake's variable timestep**: id Software's Quake engine introduced delta-time-based physics to handle varying framerates. This became the dominant paradigm for PC games, but introduced subtle non-determinism.
- **2003 — Glenn Fiedler's "Fix Your Timestep!"**: This article became the definitive reference for the hybrid approach: fixed-timestep logic with interpolated rendering. It formalized what console developers had always known — deterministic simulation requires fixed updates.
- **Present**: Nearly all modern engines (Unity, Unreal, Godot) separate the physics/logic update rate (fixed) from the render rate (variable), following Fiedler's model. bit-bound uses the simpler pure-fixed approach to match the retro-console philosophy.

## Alternatives

| Approach                            | Description                                                                                               | Trade-off                                                                          |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| **Variable timestep**               | Multiply everything by `dt`. Runs as fast as hardware allows.                                             | Smooth on fast hardware, but non-deterministic. Physics bugs at extreme framerates |
| **Semi-fixed timestep**             | Cap `dt` to a maximum (e.g., 1/30s) to prevent physics explosions.                                        | Band-aid — still non-deterministic within the cap range                            |
| **Fixed + interpolation** (Fiedler) | Logic at fixed rate, rendering interpolates between states for visual smoothness.                         | Best of both worlds, but complex. Requires storing two states (previous + current) |
| **Lockstep networking**             | Fixed timestep is mandatory. All clients step identically. Used in RTS games (StarCraft, Age of Empires). | Determinism is not optional — it's the architecture                                |

## Further Reading

- [Glenn Fiedler — "Fix Your Timestep!"](https://gafferongames.com/post/fix_your_timestep/) — The definitive article. If you read one thing, read this
- [Bob Nystrom — "Game Loop" (Game Programming Patterns)](https://gameprogrammingpatterns.com/game-loop.html) — Covers all loop variants with clear diagrams
- [NES frame timing](https://www.nesdev.org/wiki/Cycle_reference_chart) — How the NES locked logic to the CRT's vsync
- [Gaffer on Games — Networked Physics](https://gafferongames.com/categories/networked-physics/) — Why determinism matters for multiplayer
