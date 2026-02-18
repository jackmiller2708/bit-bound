pub mod enemy;
pub mod fixed_pool;
pub mod level;
pub mod player;

use crate::game::enemy::Enemy;
use crate::game::fixed_pool::FixedPool;
use crate::game::player::Player;
use crate::renderer::framebuffer::{FrameBuffer, HEIGHT, WIDTH};
use crate::runtime::memory::RuntimeMemory;

// const SPRITE_W: i32 = 35;

// const SHIP_BODY_W: i32 = 16;
// const FLAME_W: i32 = SPRITE_W - SHIP_BODY_W;

pub struct GameState {
    pub player: Player,
    pub enemies: FixedPool<Enemy, 32>,
    pub spawn_timer: u32,
    pub frame_counter: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player {
                x: 2,
                y: (HEIGHT / 2) - 8, // Center the 16x16 sprite
                anim_timer: 0,
            },
            enemies: FixedPool::new(Enemy { x: 0, y: 0, vx: 0 }),
            spawn_timer: 0,
            frame_counter: 0,
        }
    }
}

pub fn update(state: &mut GameState, _memory: &mut RuntimeMemory) {
    // Update player animation
    state.player.anim_timer += 1;
    state.frame_counter += 1;

    if state.player.anim_timer >= 60 {
        state.player.anim_timer = 0;
    }

    // Spawn enemy every 30 frames
    state.spawn_timer += 1;

    if state.spawn_timer >= 30 {
        state.spawn_timer = 0;

        let _ = state.enemies.spawn(Enemy {
            x: (WIDTH - 1) as i32,
            y: (HEIGHT / 2) as i32,
            vx: -1,
        });
    }

    // Update enemies
    let mut i = 0;

    while i < state.enemies.len() {
        let enemy = &mut state.enemies.as_mut_slice()[i];
        enemy.x += enemy.vx;

        if enemy.x < 0 {
            state.enemies.despawn(i);
            // do NOT increment i
            // swapped element now sits at i
        } else {
            i += 1;
        }
    }
}

pub fn render(state: &GameState, framebuffer: &mut FrameBuffer) {
    framebuffer.clear(0);

    let frame = if state.player.anim_timer % 20 < 10 {
        &player::PLAYER_SPRITE_F1
    } else {
        &player::PLAYER_SPRITE_F2
    };

    framebuffer.draw_sprite(state.player.x as i32, state.player.y as i32, frame, 35, 16);

    // Enemies
    for enemy in state.enemies.as_slice() {
        if enemy.x >= 0 && enemy.x < WIDTH as i32 {
            framebuffer.set_pixel(enemy.x as usize, enemy.y as usize, 2);
        }
    }
}
