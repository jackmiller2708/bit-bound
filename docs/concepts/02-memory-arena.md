# Memory Arena Allocation

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially sections on memory, the stack/heap/static distinction, and why low-level programs care about where data lives.
>
> **Related ADRs**: [0001 — Fixed Memory Arena Allocation](../adr/0001-memory-arena-allocation.md), [0005 — Slice Allocation Support](../adr/0005-slice-allocation-support.md)

---

## What Is a Memory Arena?

A **memory arena** (also called a **bump allocator**, **linear allocator**, or **region-based allocator**) is one of the simplest and most efficient memory management strategies. The idea is straightforward:

1. **Grab a big chunk of memory up front** — before your program does any real work
2. **Serve allocations by moving a pointer forward** — each new allocation gets the next available slot
3. **Free everything at once** — instead of freeing individual objects, you reset the pointer back to the start

```
┌──────────────────────────────────────────────┐
│          Pre-allocated Memory Block          │
├──────┬──────┬──────┬─────────────────────────┤
│ Obj1 │ Obj2 │ Obj3 │     Free Space →        │
└──────┴──────┴──────┴─────────────────────────┘
                      ↑
                   pointer (next allocation goes here)
```

Think of it like a notepad. You write things sequentially, page after page. You never erase individual pages — when you're done, you tear off all the used pages at once and start fresh.

### Why Is It Called an "Arena"?

The term comes from the analogy of a gladiatorial arena — a bounded, enclosed space where all the action happens. All allocations live within the arena's boundaries, and when the arena is "cleared," everything inside is discarded at once.

---

## The Problem: Why Not Just Allocate Normally?

In most high-level languages, you never think about memory allocation. When you create a list, object, or string, the language runtime allocates memory for you, and a **garbage collector** frees it later. Under the hood, these languages use a **general-purpose heap allocator**, which is a system that manages a large pool of memory and hands out chunks on demand.

This works well for most applications, but it has three problems that are critical in real-time systems like games:

### Problem 1: Fragmentation

When you allocate and free many objects of different sizes, the heap becomes **fragmented** — full of small, scattered gaps that are individually too small to use:

```
After many alloc/free cycles:

┌────┬──┬────────┬──┬──┬──────┬──┬────┐
│Used│  │  Used  │  │  │ Used │  │Used│
└────┘  └────────┘  │  └──────┘  └────┘
      ↑             ↑           ↑
   4 bytes       2 bytes     3 bytes
   (too small    (too small  (too small
    for a 10     for a 10    for a 10
    byte alloc)  byte alloc) byte alloc)

Total free: 9 bytes. But you can't allocate 10 bytes!
```

The memory is there, but it's scattered into unusable pieces. This gets worse over time in long-running programs.

### Problem 2: Unpredictable Timing

A general-purpose allocator must **search** for a free block that fits the requested size. Sometimes this is fast (the first free block fits). Sometimes it's slow (it must scan the entire free-list, merge adjacent blocks, or even request more memory from the operating system). In a game that must complete every frame in 16.67 milliseconds, even a 1ms allocation spike is noticeable.

### Problem 3: Per-Object Cleanup

With a general-purpose allocator, you must track the lifetime of every allocation and free it individually. This is error-prone — you might:
- **Forget to free** → memory leak (memory grows forever)
- **Free too early** → dangling pointer (crash or corruption)
- **Free twice** → undefined behavior (crash)

Garbage collectors solve this but introduce their own unpredictable pauses.

### How Arenas Solve All Three

| Problem                 | General-Purpose Heap   | Arena                                       |
| ----------------------- | ---------------------- | ------------------------------------------- |
| Fragmentation           | Gets worse over time   | **Impossible** — allocations are contiguous |
| Allocation speed        | Variable (must search) | **Constant** — just bump a pointer forward  |
| Deallocation complexity | Per-object tracking    | **One operation** — reset the pointer       |

---

## How It Works

### The Bump Allocator — Step by Step

An arena is backed by a contiguous byte array. It maintains a single `used` counter that tracks how much of the array has been consumed:

```rust
struct Arena {
    buffer: [u8; CAPACITY],  // The raw memory block
    used: usize,             // How many bytes have been allocated
}
```

When you request an allocation:

```
fn alloc<T>(&mut self) -> &mut T {
    1.  Calculate the size needed: size = size_of::<T>()

    2.  Handle alignment:
        - CPUs access memory most efficiently when data is aligned
          to its natural boundary (e.g., a 4-byte integer should start
          at an address divisible by 4)
        - Round up `self.used` to the next multiple of align_of::<T>()

    3.  Check if there's enough room:
        - If used + size > CAPACITY → out of memory! Return an error.

    4.  Record the current position as the start of the allocation

    5.  Advance the pointer: self.used += size

    6.  Return a mutable reference to the allocated region
}
```

### Visual Walkthrough

Let's trace three allocations in a 32-byte arena:

```
Starting state:
  buffer: [________________________] (32 bytes, all unused)
  used: 0

After alloc::<u32>() — allocates 4 bytes:
  buffer: [AAAA____________________]
  used: 4

After alloc::<u8>() — allocates 1 byte:
  buffer: [AAAAB___________________]
  used: 5

After alloc::<[u16; 4]>() — allocates 8 bytes (may need alignment padding):
  buffer: [AAAAB_CCCCCCCC__________]
  used: 14  (1 byte of padding was added for alignment)

Reset:
  buffer: [________________________] (data still there, but pointer is back to 0)
  used: 0  ← all allocations "freed" instantly
```

> **Note on alignment**: When we allocated the `[u16; 4]` array after a `u8`, there was 1 byte of padding added. A `u16` needs to start at an even address (divisible by 2). The `used` pointer was at 5 (odd), so the arena bumped it to 6 (even) before allocating. This padding is a small cost of maintaining correct alignment. See [Foundations](00-foundations.md#3-memory--a-giant-numbered-grid) for more on how memory addresses work.

### Slice Allocation

Sometimes you need a contiguous block of multiple identical items — for example, an array of 32 enemies, or a tilemap of 160 tiles. The arena supports this with `alloc_slice`:

```rust
fn alloc_slice<T>(&mut self, count: usize) -> &mut [T] {
    // Same as alloc, but for count × size_of::<T>() bytes
    // Returns a mutable slice (&mut [T]) instead of a single reference
}
```

This is how bit-bound allocates arrays without using `Vec` or any heap-based dynamic container. The size must be known at allocation time — you can't grow the slice later. For more on why fixed-size containers are a deliberate choice, see [Fixed-Capacity Entities](07-fixed-capacity-entities.md).

---

## Multi-Arena Partitioning

A single arena works for simple cases, but bit-bound uses **three separate arenas**, each for data with different lifetimes:

```
┌────────────────────────────────────────────────────────────┐
│                  RuntimeMemory (1 MB total)                 │
├──────────────────┬─────────────────────┬───────────────────┤
│  Global Arena    │    Level Arena      │   Frame Arena     │
│    (256 KB)      │     (512 KB)        │    (256 KB)       │
├──────────────────┼─────────────────────┼───────────────────┤
│ Lives forever:   │ Lives for one level:│ Lives for one     │
│ - Save data      │ - Tilemap           │ frame (16.67ms):  │
│ - Settings       │ - Enemy data        │ - Temp strings    │
│ - Meta state     │ - Level assets      │ - Physics scratch │
│                  │                     │ - Debug info      │
│ Never reset      │ Reset on level      │ Reset every tick  │
│ during gameplay  │ transition          │ (60× per second)  │
└──────────────────┴─────────────────────┴───────────────────┘
```

This partitioning is the key insight: **data has different lifetimes, so it belongs in different arenas**.

### Why Three Arenas?

| Arena      | When It's Reset             | What Goes In It                                | Why Separate?                                                                 |
| ---------- | --------------------------- | ---------------------------------------------- | ----------------------------------------------------------------------------- |
| **Global** | Never (until program exits) | Player progress, settings                      | This data must survive level transitions                                      |
| **Level**  | When changing levels        | Tile maps, enemy arrays, level-specific assets | When you leave a level, ALL its data is irrelevant — free it in one operation |
| **Frame**  | Every single tick (60×/sec) | Temporary calculations, debug strings          | Scratch space reused every frame — free it automatically                      |

Without this separation, you'd need to track which allocations are "level data" vs "global data" and free them individually. The partition makes it automatic: reset the Level arena = all level data gone. Reset the Frame arena = all temporary data gone. See [Fixed Timestep](06-fixed-timestep.md) for how this connects to the game loop.

---

## History

Arena allocation traces back to the earliest days of computing, but became a formalized pattern through decades of real-world use:

- **1960s–70s — Early mainframes**: Programs were given fixed memory partitions by the OS. There was no sophisticated heap; you managed your own slab of memory manually. This is the conceptual ancestor of arenas.
- **1976 — Region-based memory**: Academic work on "regions" (notably by Hanson, later formalized by Tofte and Talpin in the 1990s) established the theory: group related allocations together, free them all at once.
- **1980s–90s — Game consoles (NES, SNES, Game Boy)**: These systems had no general-purpose heap. Developers manually carved up fixed RAM into purpose-specific blocks — essentially hand-rolled arenas. See [Hardware-Constrained Design](01-hardware-constrained-design.md) for more on these constraints.
- **1996 — Quake engine (id Software)**: John Carmack's Quake used a "Hunk" allocator — a large contiguous block with a bump pointer. This is one of the most famous real-world arena implementations and heavily influenced game engine architecture.
- **2000s–present**: Arena allocators are standard in game engines (Unity's `NativeArray`, Unreal's `FMemStackBase`), compilers (Rust's own `rustc` uses arenas extensively), and high-performance servers.

---

## Where the Arena Lives in Memory

The arena's backing buffer must live **somewhere**. In bit-bound, it lives in **static memory** — baked into the program's binary. This avoids both stack overflow (the buffer is too large for the stack) and heap allocation (which is forbidden). See [Static Memory Patterns](03-static-memory-patterns.md) for the full explanation of why and how this works.

---

## Alternatives

| Approach                                                       | Description                                                                          | Trade-off                                                                                                                             |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| **General-purpose heap** (`malloc`/`free`, Rust's `Box`/`Vec`) | The default in most languages. Dynamic sizing, per-object free.                      | Fragmentation, unpredictable latency, lifetime tracking burden                                                                        |
| **Pool allocator**                                             | Pre-allocate fixed-size slots for one type. Free individual slots via a free-list.   | Good for same-sized objects (entities), but can't handle variable sizes. See [Fixed-Capacity Entities](07-fixed-capacity-entities.md) |
| **Stack allocator**                                            | Like an arena but supports LIFO deallocation — you can free the *last* allocation.   | Useful when allocations nest naturally, but fragile if free-order is violated                                                         |
| **Slab allocator** (Linux kernel)                              | Caches frequently-allocated object sizes. Reduces fragmentation for known hot types. | Complex to implement, overkill for small embedded-style systems                                                                       |
| **Garbage collection** (Go, Java, C#, JavaScript)              | Runtime automatically tracks and frees unreachable objects.                          | Zero manual management, but GC pauses are harmful for real-time applications                                                          |

## Further Reading

- [Ryan Fleury — "Untangling Lifetimes: The Arena Allocator"](https://www.rfleury.com/p/untangling-lifetimes-the-arena-allocator) — Excellent deep-dive into arena philosophy
- [Quake's Hunk allocator source](https://github.com/id-Software/Quake/blob/master/WinQuake/zone.c) — The OG game engine arena
- [Tofte & Talpin — "Region-Based Memory Management"](https://doi.org/10.1006/inco.1997.2706) — The academic foundation
- [Rust `bumpalo` crate](https://docs.rs/bumpalo/latest/bumpalo/) — A production-quality arena allocator for Rust
