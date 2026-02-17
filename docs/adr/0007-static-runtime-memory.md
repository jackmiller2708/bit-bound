# 7. Static Runtime Memory Pool

Date: 2026-02-17

## Status
Accepted

## Context
The default stack size on Windows (approx 1MB) is insufficient to hold the `RuntimeMemory` struct (1MB + overhead) and other local variables, leading to a `STATUS_STACK_OVERFLOW` during initialization in `main`.

While ADR 0001 forbids dynamic heap allocation in game logic, we need a way to store our fixed memory pool outside the stack.

## Decision
We will move the `RuntimeMemory` and `FrameBuffer` allocations to `static` memory.
- `Arena::new()` and `RuntimeMemory::new()` will be converted to `const fn` to allow static initialization.
- The 1MB memory pool will reside in the data segment of the executable.

## Consequences
- **Positive**: Resolves stack overflow, keeps runtime deterministic and avoids implicit allocator dependency, and adheres to the "no heap/no dynamic allocation" philosophy.
- **Negative**: Increases the binary size by approximately 1MB.
