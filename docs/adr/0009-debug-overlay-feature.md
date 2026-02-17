# 9. Debug Overlay Feature

Date: 2026-02-17

## Status
Accepted

## Context
During development, we need visibility into runtime performance and memory usage without compromising the "no heap allocation" philosophy or polluting production builds. Traditional logging to stdout is insufficient for real-time monitoring in a graphical application.

## Decision
We will implement an optional debug overlay system:
- Gated behind a `debug_overlay` Cargo feature flag
- Renders directly to the framebuffer using a minimal 3x5 pixel font
- Displays FPS and memory usage for all three arenas (global, level, frame)
- Uses compile-time conditional compilation (`#[cfg(feature = "debug_overlay")]`)
- Implemented in a separate `runtime::debug` module

### Font Specifications
- **Dimensions**: 3 pixels wide × 5 pixels tall
- **Font advance**: 4 pixels (3 width + 1 spacing)
- **Line height**: 6 pixels
- **Character set**: Digits (0-9), uppercase letters (A-Z), special characters (`:`, `/`, space)

### Layout
- **Position**: Top-left corner with 2px left padding
- **Metrics displayed**: FPS, Global memory (G), Level memory (L), Frame memory (F)
- **Total width**: ~90 pixels (fits comfortably in 160px screen)

### Implementation Details
- `FrameBuffer::draw_u32()`: Renders fixed-width numeric values
- `FrameBuffer::draw_text()` and `draw_char()`: Render text using font glyphs
- `runtime::debug::DebugInfo`: Struct holding debug metrics
- `runtime::debug::render_debug_overlay()`: Renders the overlay

## Consequences
- **Positive**: Zero runtime cost when feature is disabled, real-time performance visibility, no heap allocation for debug info, minimal screen footprint
- **Negative**: Debug overlay consumes ~7 pixels at the top of the screen, adds minor complexity to main loop
