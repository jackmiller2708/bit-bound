# 8. Global Wrapper Pattern for Static Memory

Date: 2026-02-17

## Status
Accepted

Amends [7. Static Runtime Memory Pool](0007-static-runtime-memory.md)

## Context
ADR 0007 introduced `static mut` for the runtime memory pool to avoid stack overflow. While this works in a single-threaded context, `static mut` bypasses Rust's borrow checker entirely, creating potential for undefined behavior if:
- Threads are introduced later
- Multiple mutable borrows occur in the same scope
- Ownership patterns change during refactoring

The `static mut` pattern requires `#[allow(static_mut_refs)]` and offers no compile-time safety guarantees.

## Decision
We will wrap static memory in a `Global<T>` type using `UnsafeCell<T>`.
- `Global<T>` encapsulates the unsafe access pattern
- Provides a single, controlled access point via `.get() -> &mut T`
- Still uses static storage (no heap allocation)
- Makes the "single-threaded, controlled access" contract explicit in the type system

## Consequences
- **Positive**: Clearer safety boundary, no `static mut` keyword, easier to audit unsafe code, maintains "no heap" philosophy.
- **Negative**: Still requires `unsafe` internally, but it's localized to the `Global` implementation rather than scattered throughout `main`.
