use crate::renderer::framebuffer::{FONT_ADVANCE, FrameBuffer};

const DEBUG_Y: usize = 1;
const DEBUG_X_PADDING: usize = 2;

const COL_FPS: usize = 0;
const COL_GLOBAL: usize = 7;
const COL_LEVEL: usize = 12;
const COL_FRAME: usize = 17;

fn col_to_x(col: usize) -> usize {
    DEBUG_X_PADDING + col * FONT_ADVANCE
}

pub struct DebugInfo {
    pub frame_us: u32,
    pub global_used: u32,
    pub level_used: u32,
    pub frame_used: u32,
}

pub fn render_debug_overlay(framebuffer: &mut FrameBuffer, info: &DebugInfo) {
    let fps = if info.frame_us > 0 {
        1_000_000 / info.frame_us
    } else {
        0
    };

    // Labels
    framebuffer.draw_text(col_to_x(COL_FPS), DEBUG_Y, "FPS", 2);
    framebuffer.draw_text(col_to_x(COL_GLOBAL), DEBUG_Y, "G", 2);
    framebuffer.draw_text(col_to_x(COL_LEVEL), DEBUG_Y, "L", 2);
    framebuffer.draw_text(col_to_x(COL_FRAME), DEBUG_Y, "F", 2);

    // Values
    framebuffer.draw_u32(col_to_x(COL_FPS) + 3 * FONT_ADVANCE, DEBUG_Y, fps, 3, 3);
    framebuffer.draw_u32(
        col_to_x(COL_GLOBAL) + FONT_ADVANCE,
        DEBUG_Y,
        info.global_used,
        3,
        3,
    );
    framebuffer.draw_u32(
        col_to_x(COL_LEVEL) + FONT_ADVANCE,
        DEBUG_Y,
        info.level_used,
        3,
        3,
    );
    framebuffer.draw_u32(
        col_to_x(COL_FRAME) + FONT_ADVANCE,
        DEBUG_Y,
        info.frame_used,
        3,
        3,
    );
}
