# 3. Deterministic Fixed Timestep

Date: 2026-02-17

## Status
Accepted

## Context
Variable framerates can lead to inconsistent physics, subtle bugs in game logic, and non-deterministic behavior. To emulate the "always-on" nature of classic hardware and ensure a consistent experience, we need a predictable update cycle.

## Decision
The runtime will operate on a strict 60 FPS fixed timestep.
- Each tick represents exactly 1/60th of a second (approx. 16.67ms).
- Game logic is updated exactly once per tick.
- The frame memory arena is reset at the start of every tick.

## Consequences
- **Positive**: Simplified physics calculations (no `dt` multiplication required), guaranteed identical behavior across different host hardware, and easier recording/replay functionality.
- **Negative**: If logic takes longer than 16.6ms, the game will slow down visually ("lag") rather than skipping updates.
