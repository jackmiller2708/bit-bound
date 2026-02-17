# 1. Fixed Memory Arena Allocation

Date: 2026-02-17

## Status
Accepted

## Context
Standard Rust development often relies heavily on heap allocation (`Vec`, `Box`, `String`, `Arc`, etc.). However, in constrained game environments, heap fragmentation and unpredictable allocation timing can lead to performance issues and architectural sloppiness.

We want to emulate early hardware environments where memory was a scarce, fixed resource.

## Decision
We will use a fixed memory arena system for all runtime allocations.
- The total memory pool is capped at 1 MB.
- Allocation will be performed through a `RuntimeMemory` structure managing three specific arenas:
  - **Global Arena**: (256 KB) Persistent state.
  - **Level Arena**: (512 KB) Level-specific data, reset on level transition.
  - **Frame Arena**: (256 KB) Scratchpad for per-frame calculations, reset every tick.
- Dynamic containers from the standard library (e.g., `Vec`, `HashMap`) are forbidden in the core runtime logic.
- The `Arena` implementation will use a simple pointer-bump allocator pattern.

## Consequences
- **Positive**: Zero heap fragmentation, deterministic allocation performance, explicit memory monitoring, and forced architectural discipline.
- **Negative**: Increased complexity for the developer to manage object lifetimes and pre-determine sizes. Running out of memory will result in a hard failure (panic or handled error) rather than a dynamic resize.
