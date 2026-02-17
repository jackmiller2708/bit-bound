use bit_bound::game::{GameState, render, update};
use bit_bound::memory::RuntimeMemory;
use bit_bound::renderer::framebuffer::{self, FrameBuffer};

use std::cell::UnsafeCell;
use std::time::{Duration, Instant};

use minifb::{Window, WindowOptions};

const FRAME_TIME: Duration = Duration::from_millis(16);

struct Global<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    fn get(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}

static MEMORY: Global<RuntimeMemory> = Global::new(RuntimeMemory::new());
static FRAMEBUFFER: Global<FrameBuffer> = Global::new(FrameBuffer::new());

fn main() {
    let memory = MEMORY.get();
    let buffer = FRAMEBUFFER.get();

    #[cfg(feature = "debug_overlay")]
    let mut last_frame_us = 0; // Used only for debugging

    let mut state = GameState::new();
    let mut window = Window::new(
        "BitBound",
        framebuffer::WIDTH,
        framebuffer::HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    window.set_target_fps(60);

    let mut screen_buffer = vec![0u32; framebuffer::WIDTH * framebuffer::HEIGHT];

    loop {
        let frame_start = Instant::now();

        update(&mut state, memory);
        render(&state, buffer);

        #[cfg(feature = "debug_overlay")]
        {
            use bit_bound::runtime::debug::{DebugInfo, render_debug_overlay};

            let info = DebugInfo {
                frame_us: last_frame_us,
                global_used: memory.global.used() as u32,
                level_used: memory.level.used() as u32,
                frame_used: memory.frame.used() as u32,
            };

            render_debug_overlay(buffer, &info);
        }

        buffer.to_rgba_buffer(&mut screen_buffer);

        window
            .update_with_buffer(&screen_buffer, framebuffer::WIDTH, framebuffer::HEIGHT)
            .unwrap();

        if !window.is_open() {
            break;
        }

        memory.frame.reset();

        let elapsed = frame_start.elapsed();

        #[cfg(feature = "debug_overlay")]
        {
            last_frame_us = elapsed.as_micros() as u32;
        }

        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }
}
