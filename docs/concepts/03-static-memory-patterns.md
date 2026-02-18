# Static Memory & Global Access Patterns

> **Prerequisites**: Read [Foundations](00-foundations.md) first — especially the section on the stack, the heap, and static memory. Also helpful: [Memory Arena](02-memory-arena.md) for understanding what data the static memory holds.
>
> **Related ADRs**: [0007 — Static Runtime Memory Pool](../adr/0007-static-runtime-memory.md), [0008 — Global Wrapper Pattern](../adr/0008-global-wrapper-pattern.md)

---

## The Problem: Where Does the Arena Live?

The [Memory Arena](02-memory-arena.md) doc explains that bit-bound uses a 1 MB memory pool split into three arenas. But that document doesn't answer a critical question: **where in memory does this 1 MB pool itself live?**

As covered in [Foundations](00-foundations.md#4-the-stack-the-heap-and-static-memory), there are three places data can live: the **stack**, the **heap**, or **static memory**. Each has different rules. Let's walk through why each option does or doesn't work for a 1 MB memory pool.

### Option 1: The Stack ❌

```rust
fn main() {
    let memory = RuntimeMemory::new();  // 1 MB on the stack
    // ...
}
```

The stack has a fixed, limited size — typically 1 MB on Windows, 8 MB on Linux. If you try to place a 1 MB struct on the stack, you'll consume the entire stack immediately, leaving no room for function calls, local variables, or anything else. The result: **stack overflow** — the program crashes before it even starts.

### Option 2: The Heap ❌

```rust
fn main() {
    let memory = Box::new(RuntimeMemory::new());  // Allocates on the heap
    // ...
}
```

This would work technically, but bit-bound's entire philosophy forbids heap allocation (see [Memory Arena](02-memory-arena.md) — the arena system exists specifically to avoid the heap). Using `Box` to allocate the thing that's supposed to replace the heap would be contradictory.

### Option 3: Static Memory ✅

```rust
static MEMORY: RuntimeMemory = RuntimeMemory::new();
```

**Static memory** (the data segment) is part of the compiled binary. It exists before `main()` runs, requires no allocation at runtime, and lives for the entire program's lifetime. It has no size limit beyond the binary file size. This is the solution bit-bound uses.

---

## What Is Static Memory?

When a program is compiled, the resulting binary (the `.exe` file) contains several **segments** — regions of memory with specific purposes:

```
The compiled binary, loaded into RAM:

┌─────────────────────┐  High addresses
│       Stack         │  ← Local variables. Grows downward.
│         ↓           │     Limited size (1-8 MB).
│                     │
│         ↑           │
│        Heap         │  ← Dynamic allocations (Box, Vec, malloc).
│                     │     Grows upward. Size limited by OS.
├─────────────────────┤
│   .bss              │  ← Uninitialized static variables.
│   (zero-initialized)│     bit-bound's arena buffer lives here.
├─────────────────────┤
│   .data             │  ← Initialized static variables.
│   (has values)      │     e.g., static X: i32 = 42
├─────────────────────┤
│   .text             │  ← The compiled machine instructions.
│   (your code)       │     Read-only.
└─────────────────────┘  Low addresses
```

### `.bss` vs `.data`

- **`.data`** segment: For static variables with non-zero initial values. The initial values are stored in the binary file. `static X: i32 = 42` lives here.
- **`.bss`** segment: For static variables initialized to zero. These don't store actual values in the binary — the OS zeros the memory when the program starts. `static BUFFER: [u8; 1_048_576] = [0; 1_048_576]` lives here.

bit-bound's arena buffer is zero-initialized, so it goes in `.bss`. This means the 1 MB buffer **doesn't increase the binary file size by 1 MB** — the binary just records "reserve 1 MB of zeroed memory at startup," and the OS allocates it when the program loads.

### Key Properties of Static Memory

| Property            | Description                                                              |
| ------------------- | ------------------------------------------------------------------------ |
| **Lifetime**        | Exists from program start to program exit. Never freed during execution. |
| **Allocation cost** | Zero — it's already there when `main()` begins.                          |
| **Size**            | Fixed at compile time. Cannot grow or shrink.                            |
| **Binary impact**   | `.data` items increase binary size. `.bss` items do not (only metadata). |
| **Availability**    | Accessible from anywhere in the program (it's "global").                 |

---

## The Mutability Problem

There's a catch. In Rust, `static` variables are **immutable by default**:

```rust
static MEMORY: RuntimeMemory = RuntimeMemory::new();

fn main() {
    MEMORY.global_arena.alloc::<Player>();  // ERROR: Can't mutate a static
}
```

An arena allocator must mutate its internal state (advancing the `used` pointer). So we need a way to make static data **mutable**.

### Attempt 1: `static mut` — It Works, But It's Dangerous

Rust does have `static mut`:

```rust
static mut MEMORY: RuntimeMemory = RuntimeMemory::new();

fn main() {
    unsafe {
        MEMORY.global_arena.alloc::<Player>();  // Works! But requires `unsafe`.
    }
}
```

Every access to `static mut` requires an `unsafe` block because Rust can't guarantee that two pieces of code won't mutate it simultaneously. If that happened (e.g., in multi-threaded code), it would be **undefined behavior** — the program might crash, corrupt data, or produce wrong results.

The problems with `static mut`:
- **`unsafe` is scattered everywhere** — every function that touches the memory needs `unsafe`
- **No compile-time protection** — Rust's borrow checker can't help you; you're on your own
- **Fragile** — if you ever add threads or concurrent access, nothing warns you

### Attempt 2: `Global<T>` Wrapper — Concentrated Safety

bit-bound's solution is to wrap the static data in a `Global<T>` type that **concentrates the unsafety** into one small, auditable location:

```rust
use core::cell::UnsafeCell;

pub struct Global<T> {
    inner: UnsafeCell<T>,
}
```

### What Is `UnsafeCell`?

`UnsafeCell` is Rust's most fundamental building block for **interior mutability** — the ability to mutate data even when you only have a shared (immutable) reference to it.

Normally, Rust's rules are strict: if you have a `&T` (shared reference), you cannot modify the data. `UnsafeCell<T>` is the one and only exception. It tells the compiler: "I take responsibility for ensuring safe mutation. Don't optimize based on immutability assumptions."

Every safe interior-mutability type in Rust (`RefCell`, `Mutex`, `RwLock`, `Cell`) is built on top of `UnsafeCell`. `Global<T>` is bit-bound's own minimal wrapper.

### How `Global<T>` Works

```rust
use core::cell::UnsafeCell;

pub struct Global<T> {
    inner: UnsafeCell<T>,
}

// Tell Rust this type can be stored in a `static`.
// SAFETY: We guarantee single-threaded access only.
unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    /// Create a new Global. This is a `const fn` so it can be
    /// called in a `static` initializer — no runtime code needed.
    pub const fn new(value: T) -> Self {
        Global {
            inner: UnsafeCell::new(value),
        }
    }

    /// Get a mutable reference to the inner value.
    /// SAFETY: Caller must ensure no other code is simultaneously
    /// accessing this Global.
    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}
```

### Usage

```rust
// Declared once as a static. Lives in .bss segment.
static MEMORY: Global<RuntimeMemory> = Global::new(RuntimeMemory::new());

fn main() {
    // Access is through a single, controlled method.
    // No `static mut`, no `unsafe` at the call site.
    let mem = MEMORY.get();
    mem.global_arena.alloc::<Player>();
}
```

### Why Is This Better Than `static mut`?

| Aspect                     | `static mut`                              | `Global<T>`                                         |
| -------------------------- | ----------------------------------------- | --------------------------------------------------- |
| Unsafety location          | Scattered across every access site        | Concentrated in one `get()` method                  |
| Borrow checker involvement | None — compiler gives up                  | The `&mut T` return type still provides some safety |
| Auditability               | Must review every `unsafe` block          | Review only the `Global` implementation             |
| Change impact              | Adding threads breaks everything silently | The `unsafe impl Sync` is the single point to audit |

---

## Why Not Use Existing Solutions?

Rust's ecosystem has several established patterns for global mutable state. bit-bound doesn't use them, and here's why:

### `lazy_static!` / `LazyLock`

These initialize a global value lazily (on first access) and use a `Mutex` internally for thread safety:

```rust
lazy_static! {
    static ref MEMORY: Mutex<RuntimeMemory> = Mutex::new(RuntimeMemory::new());
}
```

**Why not**: This heap-allocates internally, adds `Mutex` lock/unlock overhead on every access (60× per second × multiple accesses per tick = thousands of unnecessary lock operations), and is conceptually wrong for a single-threaded program.

### `RefCell<T>`

Runtime-checked borrowing. Panics if you borrow mutably while already borrowed:

```rust
static MEMORY: RefCell<RuntimeMemory> = RefCell::new(RuntimeMemory::new());
MEMORY.borrow_mut().global_arena.alloc::<Player>();
```

**Why not**: `RefCell` can't be placed in a `static` directly (it's not `Sync`), and it adds a runtime borrow counter that's checked on every access — unnecessary overhead when you can guarantee single-threaded access.

### Dependency Injection

Pass the memory as a parameter through every function:

```rust
fn update_game(memory: &mut RuntimeMemory) {
    let player = update_player(&mut memory.global_arena);
    let enemies = update_enemies(&mut memory.level_arena);
    render(&mut memory.frame_arena, &framebuffer);
}
```

**Why not**: This is the "most correct" approach from a software engineering perspective, but in a game loop with deeply nested call chains, threading a parameter through every function, every frame, creates significant noise. The `Global` wrapper trades some theoretical safety for practical ergonomics.

---

## Connection to the System

Static memory and the `Global<T>` wrapper are the **foundation layer** that everything else builds on:

```
Global<RuntimeMemory>  (static, .bss segment)
  └─ RuntimeMemory
       ├─ Global Arena (256 KB)   → see Memory Arena
       ├─ Level Arena (512 KB)    → see Memory Arena
       └─ Frame Arena (256 KB)    → see Memory Arena (reset each tick)

Global<FrameBuffer>    (static, .bss segment)
  └─ FrameBuffer (5,760 bytes)   → see FrameBuffer & Bit-Packing
       └─ draw_sprite()          → see Sprite Rendering
```

The [Fixed Timestep](06-fixed-timestep.md) game loop accesses these globals each tick. The [Debug Overlay](08-debug-overlays.md) reads arena usage stats from them. The entire system depends on this layer.

---

## History

- **1950s–60s — Early computers**: All variables were effectively static — programs were loaded at fixed memory locations. FORTRAN (1957) stored all variables statically, which is why early FORTRAN didn't support recursion (recursion requires the stack).
- **1970s — C language**: C formalized the distinction between `static` (data segment), `auto` (stack), and heap (`malloc`). The `static` keyword placed variables in the data segment.
- **1980s — Embedded systems**: Microcontrollers with minimal RAM (sometimes 128 bytes) used static allocation exclusively — no heap, no dynamic dispatch. This is the closest analog to bit-bound's approach.
- **1990s — Game consoles**: Console games frequently used global state for game systems. Developers statically partitioned RAM into regions at compile time.
- **2015 — Rust's ownership model**: Rust made global mutable state intentionally difficult. `static mut` exists but provides no safety. The community developed patterns: `lazy_static!` (2014), `once_cell` (2019, later stabilized as `OnceLock`), and interior mutability wrappers. bit-bound's `Global<T>` follows this tradition with minimal complexity.

## Alternatives

| Approach                        | Description                                                          | Trade-off                                                                   |
| ------------------------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| **`static mut`**                | Built-in global mutable state. Every access requires `unsafe`.       | Works, but unsafe is scattered everywhere. Easy to create soundness bugs.   |
| **`lazy_static!` / `LazyLock`** | Lazy initialization with thread-safe `Mutex`.                        | Heap allocates, adds lock overhead. Overkill for single-threaded.           |
| **`OnceLock` / `OnceCell`**     | Write-once globals. Thread-safe. Standard library (since Rust 1.70). | Good for one-time init, not for data that mutates every frame.              |
| **`Mutex<T>`**                  | Thread-safe interior mutability. Lock/unlock per access.             | Completely unnecessary in single-threaded code. Significant overhead.       |
| **`RefCell<T>`**                | Runtime borrow checking. Panics on double-borrow.                    | Can't be `static` directly. Runtime cost (borrow counter).                  |
| **Dependency injection**        | Pass `&mut RuntimeMemory` through the call chain.                    | Safest, most idiomatic Rust. But adds parameter-threading noise everywhere. |
| **Increase stack size**         | Configure a larger stack via linker flags.                           | Sidesteps the problem. Still subject to stack limits. Fragile.              |

## Further Reading

- [Foundations — Stack, Heap, Static](00-foundations.md#4-the-stack-the-heap-and-static-memory) — The prerequisite explanation
- [The Rust Reference — Static Items](https://doc.rust-lang.org/reference/items/static-items.html) — Official documentation on `static` and `static mut`
- [Rust Nomicon — `UnsafeCell`](https://doc.rust-lang.org/nomicon/send-and-sync.html) — Why `UnsafeCell` is the foundation of all interior mutability
- [Embedded Rust — Static Guarantees](https://docs.rust-embedded.org/book/static-guarantees/index.html) — How embedded Rust handles global state safely
