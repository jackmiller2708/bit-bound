pub mod enemy;
pub mod entity;
pub mod fixed_pool;
pub mod level;

use crate::game::enemy::Enemy;
use crate::game::fixed_pool::FixedPool;
use crate::memory::RuntimeMemory;
use crate::renderer::framebuffer::{FrameBuffer, HEIGHT, WIDTH};

pub struct GameState {
    pub x: usize,
    pub y: usize,
    pub direction: i32,
    pub enemies: FixedPool<Enemy, 32>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: HEIGHT / 2,
            direction: 1,
            enemies: FixedPool::new(Enemy { x: 0, y: 0, vx: 0 }),
        }
    }
}

pub fn update(state: &mut GameState, _memory: &mut RuntimeMemory) {
    let next = state.x as i32 + state.direction;

    if next <= 0 {
        state.direction = 1;
    } else if next >= (WIDTH as i32 - 1) {
        state.direction = -1;
    }

    state.x = (state.x as i32 + state.direction) as usize;
}

pub fn render(state: &GameState, framebuffer: &mut FrameBuffer) {
    framebuffer.clear(0);
    framebuffer.set_pixel(state.x, state.y, 3);
}
