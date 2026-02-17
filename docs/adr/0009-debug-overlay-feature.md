# 9. Debug Overlay Feature

Date: 2026-02-17

## Status
Accepted

## Context
During development, we need visibility into runtime performance and memory usage without compromising the "no heap allocation" philosophy or polluting production builds. Traditional logging to stdout is insufficient for real-time monitoring in a graphical application.

## Decision
We will implement an optional debug overlay system:
- Gated behind a `debug_overlay` Cargo feature flag
- Renders directly to the framebuffer using the existing 3x5 font system
- Displays FPS and memory usage for all three arenas (global, level, frame)
- Uses compile-time conditional compilation (`#[cfg(feature = "debug_overlay")]`)
- Implemented in a separate `runtime::debug` module

### Implementation Details
- `FrameBuffer::draw_u32()`: Renders fixed-width numeric values
- `FrameBuffer::draw_text()` and `draw_char()`: Render text using font glyphs
- `runtime::debug::DebugInfo`: Struct holding debug metrics
- `runtime::debug::render_debug_overlay()`: Renders the overlay

## Consequences
- **Positive**: Zero runtime cost when feature is disabled, real-time performance visibility, no heap allocation for debug info
- **Negative**: Debug overlay consumes a few pixels at the top of the screen, adds minor complexity to main loop
