# 5. Slice Allocation Support

Date: 2026-02-17

## Status
Accepted

Amends [1. Fixed Memory Arena Allocation](0001-memory-arena-allocation.md)

## Context
The initial memory arena implementation only supported single object allocation via `alloc<T>`. However, game development frequently requires contiguous blocks of memory for arrays (e.g., tile layers, entity buffers, or particle systems).

## Decision
We will extend the `Arena` allocator with an `alloc_slice<T>(count: usize)` method.
- It will calculate the total size required for `count` elements of type `T`.
- It will handle alignment and boundary checks identically to `alloc`.
- It will return a `Result<&mut [T], MemoryError>`.

## Consequences
- **Positive**: Enables efficient contiguous memory blocks without dynamic `Vec` usage.
- **Negative**: The caller must know the count at allocation time (fixed-size buffers).
