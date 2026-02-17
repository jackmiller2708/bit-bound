pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

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

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        if x >= WIDTH || y >= HEIGHT {
            return 0;
        }

        let index = y * WIDTH + x;
        let byte_index = index / 4;
        let pixel_offset = (index % 4) * 2;

        (self.buffer[byte_index] >> pixel_offset) & 0b11
    }

    pub fn raw(&self) -> &[u8] {
        &self.buffer
    }
}
