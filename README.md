# bit-bound

A deliberately constrained game runtime designed to emulate the discipline of early game development environments.

## Vision

This project is a constrained game runtime that enforces hard limits and explicit resource management. The goal is architectural clarity through constraint, rather than modern engine abstractions.

## Core Philosophy

- **Emulate old hardware limits**: 160 × 144 resolution, 4-color palette.
- **Enforce strict memory ceilings**: 1 MB fixed total memory pool.
- **Fixed-capacity systems**: No dynamic allocation (Vec, Box, String).
- **Deterministic 60 FPS fixed timestep**: Predictable performance and behavior.
- **Explicit resource usage**: All resources are pre-allocated and measurable.
- **Punish sloppy architecture**: Hard failures when constraints are violated.

## Technical Constraints

- **Resolution**: 160 × 144 (2 bits per pixel)
- **Memory**: 1 MB (Fixed Arena System)
- **Timestep**: 60 FPS
- **Tile Size**: 16 × 16
- **Level Width**: ~200 tiles
- **Entity Limits**: 32 enemies, 64 projectiles

## Runtime Architecture

The runtime operates on a memory arena system with three primary segments:
- **Global**: Persistent state across frames and levels.
- **Level**: Assets and state loaded for a single level.
- **Frame**: Temporary memory reset every tick.

## Documentation

See the `/docs` directory for detailed architecture and ADRs.
