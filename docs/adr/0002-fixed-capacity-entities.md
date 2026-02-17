# 2. Fixed-Capacity Entity System

Date: 2026-02-17

## Status
Accepted

## Context
To ensure deterministic behavior and strictly adhere to hardware-like constraints, we must limit the number of active entities in the game world. Dynamic spawning can lead to CPU spikes and memory exhaustion if not carefully managed.

## Decision
All entity systems (enemies, projectiles, items) will use fixed-capacity storage arrays.
- **Maximum Enemies**: 32
- **Maximum Projectiles**: 64
- **Maximum Items**: 16 (estimated)

Systems will iterate over these fixed arrays. Entity "spawning" will involve finding an inactive slot in the array rather than allocating new memory.

## Consequences
- **Positive**: Constant-time "allocation" and "deallocation", bounded CPU usage per system, and predictable memory footprint.
- **Negative**: Hard limits on game design (e.g., no more than 64 bullets on screen). Developers must implement pooling logic manually.
