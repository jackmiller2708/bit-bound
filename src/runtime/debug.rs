use crate::renderer::framebuffer::FrameBuffer;

const DEBUG_Y: usize = 2;

const FPS_X: usize = 6;
const GLOBAL_X: usize = 46;
const LEVEL_X: usize = 86;
const FRAME_X: usize = 126;

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

    framebuffer.draw_text(FPS_X, DEBUG_Y, "FPS", 3);
    framebuffer.draw_text(GLOBAL_X, DEBUG_Y, "G", 3);
    framebuffer.draw_text(LEVEL_X, DEBUG_Y, "L", 3);
    framebuffer.draw_text(FRAME_X, DEBUG_Y, "F", 3);

    framebuffer.draw_u32(FPS_X + 12, DEBUG_Y, fps, 3, 3);
    framebuffer.draw_u32(GLOBAL_X + 4, DEBUG_Y, info.global_used, 3, 3);
    framebuffer.draw_u32(LEVEL_X + 4, DEBUG_Y, info.level_used, 3, 3);
    framebuffer.draw_u32(FRAME_X + 4, DEBUG_Y, info.frame_used, 3, 3);
}
