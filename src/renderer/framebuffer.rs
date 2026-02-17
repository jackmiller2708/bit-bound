use crate::renderer::font::{Glyph, get_glyph};

pub const PALETTE: [u32; 4] = [
    0xFF0F380F, // Darkest
    0xFF306230, 0xFF8BAC0F, 0xFF9BBC0F, // Lightest
];
pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub const FONT_WIDTH: usize = 3;
pub const FONT_HEIGHT: usize = 5;
pub const FONT_SPACING: usize = 1;
pub const LINE_HEIGHT: usize = 6;

pub const FONT_ADVANCE: usize = FONT_WIDTH + FONT_SPACING;

const PIXELS: usize = WIDTH * HEIGHT;
const BUFFER_SIZE: usize = PIXELS / 4; // 4 pixels per byte

pub struct FrameBuffer {
    buffer: [u8; BUFFER_SIZE],
}

impl FrameBuffer {
    pub const fn new() -> Self {
        Self {
            buffer: [0; BUFFER_SIZE],
        }
    }

    pub fn clear(&mut self, color: u8) {
        let packed =
            (color & 0b11) | ((color & 0b11) << 2) | ((color & 0b11) << 4) | ((color & 0b11) << 6);

        for byte in self.buffer.iter_mut() {
            *byte = packed;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let index = y * WIDTH + x;
        let byte_index = index / 4;
        let pixel_offset = (index % 4) * 2;

        let mask = !(0b11 << pixel_offset);
        let value = (color & 0b11) << pixel_offset;

        let byte = &mut self.buffer[byte_index];
        *byte = (*byte & mask) | value;
    }

    #[allow(dead_code)]
    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        if x >= WIDTH || y >= HEIGHT {
            return 0;
        }

        let index = y * WIDTH + x;
        let byte_index = index / 4;
        let pixel_offset = (index % 4) * 2;

        (self.buffer[byte_index] >> pixel_offset) & 0b11
    }

    #[allow(dead_code)]
    pub fn raw(&self) -> &[u8] {
        &self.buffer
    }

    pub fn to_rgba_buffer(&self, out: &mut [u32]) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color_index = self.get_pixel(x, y) as usize;
                out[y * WIDTH + x] = PALETTE[color_index];
            }
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, glyph: &Glyph, color: u8) {
        for row in 0..FONT_HEIGHT {
            let bits = glyph.rows[row];

            for col in 0..FONT_WIDTH {
                if (bits >> (7 - col)) & 1 == 1 {
                    self.set_pixel(x + col, y + row, color);
                }
            }
        }
    }

    pub fn draw_text(&mut self, mut x: usize, y: usize, text: &str, color: u8) {
        for c in text.chars() {
            if let Some(glyph) = get_glyph(c) {
                self.draw_char(x, y, glyph, color);
            }

            x += FONT_ADVANCE;
        }
    }

    pub fn draw_u32(&mut self, mut x: usize, y: usize, value: u32, digits: usize, color: u8) {
        let mut temp = [0u8; 10];
        let mut n = value;

        for i in (0..digits).rev() {
            temp[i] = (n % 10) as u8;
            n /= 10;
        }

        for i in 0..digits {
            let c = (b'0' + temp[i]) as char;

            if let Some(glyph) = get_glyph(c) {
                self.draw_char(x, y, glyph, color);
            }

            x += FONT_ADVANCE;
        }
    }
}
