# Fixed-Capacity Entity System

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially sections on memory and the stack/heap distinction. Also helpful: [Memory Arena](02-memory-arena.md) for understanding why dynamic allocation is avoided.
>
> **Related ADR**: [0002 — Fixed-Capacity Entities](../adr/0002-fixed-capacity-entities.md)

---

## What Is an Entity?

In game development, an **entity** is anything that exists in the game world — a player character, an enemy, a bullet, a treasure chest, a power-up. Each entity has data associated with it: a position (x, y), a health value, a velocity, an animation state, etc.

The fundamental question is: **how do you store and manage these entities?**

In most high-level applications, the answer is simple: use a dynamic list. Need a new enemy? Add it to the list. Enemy died? Remove it from the list. The runtime handles memory automatically.

In bit-bound, dynamic lists are forbidden (no heap allocation — see [Memory Arena](02-memory-arena.md)). Instead, entities are managed using **fixed-capacity arrays** — a pattern also known as an **object pool**.

---

## What Is a Fixed-Capacity Array?

A **fixed-capacity array** is an array whose size is decided at compile time and never changes. In Rust:

```rust
let enemies: [Enemy; 32] = [Enemy::default(); 32]; // Exactly 32 slots. Always.
```

This is fundamentally different from a dynamic list:

| Feature               | Dynamic List                               | Fixed-Capacity Array                               |
| --------------------- | ------------------------------------------ | -------------------------------------------------- |
| Size                  | Grows and shrinks at runtime               | Set at compile time, never changes                 |
| Memory allocation     | Allocates when adding, frees when removing | No allocation ever — memory is always reserved     |
| Can run out of space? | No (until system memory is exhausted)      | Yes — if all 32 slots are full, you can't add more |
| Memory location       | Heap (runtime-managed)                     | Stack or static (compile-time known)               |

---

## How Object Pooling Works

Since the array is always the same size, you can't "add" or "remove" entities in the traditional sense. Instead, every slot has an **active** flag:

```rust
struct Enemy {
    active: bool,       // Is this slot in use?
    x: i32,             // Position
    y: i32,
    health: u8,         // Hit points remaining
    enemy_type: u8,     // What kind of enemy
}
```

### "Spawning" an Entity

To spawn a new enemy, you scan the array for the first `active == false` slot and activate it:

```rust
fn spawn_enemy(enemies: &mut [Enemy; 32], x: i32, y: i32) -> Option<usize> {
    for (index, enemy) in enemies.iter_mut().enumerate() {
        if !enemy.active {
            // Found a free slot — initialize it
            enemy.active = true;
            enemy.x = x;
            enemy.y = y;
            enemy.health = 3;
            return Some(index);  // Return the slot number
        }
    }
    None  // All 32 slots are occupied. Spawn denied.
}
```

No memory is allocated. The slot was always there — you just wrote data into it and flipped `active` to `true`.

### "Despawning" an Entity

To remove an enemy (it died, went off-screen, etc.), set `active` to `false`:

```rust
fn despawn_enemy(enemies: &mut [Enemy; 32], index: usize) {
    enemies[index].active = false;
    // That's it. The memory is still there, but the slot is
    // now available for the next spawn_enemy() call.
}
```

No memory is freed. The slot is simply marked as available.

### Updating All Active Entities

Every tick, the game iterates over the entire array but only processes active slots:

```rust
fn update_enemies(enemies: &mut [Enemy; 32]) {
    for enemy in enemies.iter_mut() {
        if !enemy.active {
            continue;  // Skip empty slots
        }

        // Move the enemy
        enemy.x += 1;

        // Check if it died
        if enemy.health == 0 {
            enemy.active = false;  // "Despawn"
        }
    }
}
```

### Visual Representation

```
Index:  [ 0 ]  [ 1 ]  [ 2 ]  [ 3 ]  [ 4 ]  [ 5 ]  ...  [ 31 ]
Active:  true   true   false  true   false  false  ...   false
         ─────  ─────         ─────
         Enemy  Enemy         Enemy
         at     at            at
         (10,5) (40,8)        (90,3)

"Spawn at (60,2)" → scans array → finds slot 2 (inactive) → activates it

Index:  [ 0 ]  [ 1 ]  [ 2 ]  [ 3 ]  [ 4 ]  [ 5 ]  ...  [ 31 ]
Active:  true   true   TRUE   true   false  false  ...   false
         ─────  ─────  ─────  ─────
         Enemy  Enemy  NEW!   Enemy
```

---

## Why Not Just Use a Dynamic List?

### Reason 1: No Allocation Spikes

With a dynamic list, spawning 20 enemies in a single frame could trigger 20 separate memory allocations. Each allocation might be fast, or it might be slow (the allocator needs to search for free space, or the garbage collector needs to run). In a game that must complete each frame in 16.67ms, any unpredictable spike is dangerous.

With a fixed array, "spawning" is just writing to a pre-existing slot. It's always the same speed — effectively instant.

### Reason 2: Bounded Memory

You know **exactly** how much memory your entity system uses:

```
32 enemies × (1 + 4 + 4 + 1 + 1) bytes per enemy = 352 bytes
```

Always 352 bytes. Whether 0 enemies are active or 32, the memory usage is identical. There's no scenario where a bug spawns infinite enemies and crashes the game. The system is **self-limiting by design** — if all 32 slots are full, `spawn_enemy` simply returns `None`.

### Reason 3: Cache Friendliness

All 32 enemies sit in a contiguous block of memory — one right after another. When the CPU iterates over them, it loads them into its cache (a small, ultra-fast memory built into the processor) in sequential order. This is called **cache-friendly access** and it's dramatically faster than chasing pointers to scattered locations.

A dynamic list might store each enemy at a different memory address (especially after many add/remove cycles), forcing the CPU to "chase" pointers to different locations — each potentially a "cache miss" that costs 100+ CPU cycles.

### Reason 4: Determinism

The iteration order, memory layout, and processing cost are identical every frame, regardless of how many entities are alive. This is critical for the [Fixed Timestep](06-fixed-timestep.md) guarantee that the same inputs always produce the same outputs.

---

## bit-bound's Entity Budget

| Entity Type | Max Count | Rationale                                                         |
| ----------- | --------- | ----------------------------------------------------------------- |
| Enemies     | 32        | Screen is only 160×144 — more than 32 enemies would be unreadable |
| Projectiles | 64        | Higher density needed for bullet patterns                         |
| Items       | 16        | Pickups appear infrequently                                       |

These limits are stored in the [Level Arena](02-memory-arena.md) — when a level ends and the Level Arena resets, all entity data is freed at once.

The limits are deliberately chosen to match the spirit of [Hardware-Constrained Design](01-hardware-constrained-design.md). The NES could only display 64 sprites. The Game Boy could display 40. Fixed budgets force design discipline — you can't solve a design problem by throwing more entities at it.

---

## History

- **1970s — Arcade machines (Space Invaders, Galaxian)**: These systems had a handful of kilobytes of RAM and no operating system. Developers hardcoded fixed arrays for every entity type. Space Invaders famously had exactly 55 aliens in a fixed 5×11 grid — there was no dynamic spawning system, just a fixed array iterated each frame.
- **1980s — NES / Famicom**: The NES PPU (Picture Processing Unit) supported exactly **64 hardware sprites** with 8 per scanline. Game code managed these with fixed arrays mapped directly to hardware registers. If you've ever seen NES sprites flicker, that's the 8-per-scanline limit being exceeded.
- **1990s — Arcade fighting games and shoot-em-ups**: Games like *Street Fighter II* and *R-Type* needed deterministic frame-perfect behavior. Fixed entity tables guaranteed that spawning a projectile would never cause a frame spike from allocation.
- **2000s–present — Modern engines**: Even in engines with dynamic allocation (Unity, Unreal), **object pooling** remains a best practice for frequently spawned/destroyed objects (bullets, particles, audio sources). The technique is universal to performance-critical software.

## Alternatives

| Approach                                    | Description                                                                         | Trade-off                                                                                            |
| ------------------------------------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| **Dynamic list** (`Vec`, `ArrayList`, etc.) | Grow/shrink as needed. The default in most languages.                               | Allocation spikes, potential fragmentation, less cache-friendly                                      |
| **Free-list pool**                          | Fixed array + a linked list of free slot indices. `O(1)` spawn without scanning.    | More complex, but avoids the scan-for-empty-slot cost                                                |
| **Generational indices**                    | Pool + generation counter per slot to detect stale references safely.               | Solves the "dangling handle" problem (referencing a despawned entity's slot after it's been re-used) |
| **ECS frameworks** (Bevy, Specs, EnTT)      | Archetype or component-based storage with automatic memory management.              | Powerful and cache-friendly, but adds a framework dependency and significant conceptual overhead     |
| **Sparse sets**                             | Separate dense array (for iteration) and sparse array (for lookup). O(1) both ways. | Excellent iteration and lookup performance, but uses 2× memory per entity type                       |

## Further Reading

- [Bob Nystrom — "Object Pool" (Game Programming Patterns)](https://gameprogrammingpatterns.com/object-pool.html) — The canonical explanation
- [NES hardware sprite limitations](https://www.nesdev.org/wiki/PPU_OAM) — Why fixed slots are a hardware reality
- [Catherine West — "Using Rust for Game Development" (RustConf 2018)](https://www.youtube.com/watch?v=aKLntZcp27M) — Covers generational indices and pool patterns in Rust
