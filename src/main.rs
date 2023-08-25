extern crate sdl2;
extern crate smallvec;

use glam::{IVec2, Vec2};
use sdl2::event::Event;
use sdl2::keyboard::Scancode::*;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use smallvec::SmallVec;
use std::time::Instant;

struct Keys {
    left: Scancode,
    right: Scancode,
    up: Scancode,
    down: Scancode,
}
type Id = u16;
const VISUAL_DIAMETER: u32 = 4;
const BALL_DIAMETER: f32 = 8.;
const BUCKET_SIZE: i32 = BALL_DIAMETER as i32;
const BUCKETS_PER_ROW: i32 = LEVEL_WIDTH / BUCKET_SIZE;
const BUCKETS_PER_COLUMN: i32 = LEVEL_HEIGHT / BUCKET_SIZE;

// these should fit an even number of buckets, just in case
const LEVEL_WIDTH: i32 = 1728;
const LEVEL_HEIGHT: i32 = 1048;

const ENEMY_EVASION: f32 = 0.05f32;
const FRIENDLY_EVASION: f32 = 0.01f32;
const VELOCITY_SLOWDOWN: f32 = 0.9;
const CURSOR_FOLLOW_SPEED: f32 = 0.002f32;
const CURSOR_SPEED: f32 = 2f32;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", LEVEL_WIDTH as u32, LEVEL_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let player_keys = vec![
        Keys {
            left: A,
            right: D,
            up: W,
            down: S,
        },
        Keys {
            left: F,
            right: H,
            up: T,
            down: G,
        },
        Keys {
            left: J,
            right: L,
            up: I,
            down: K,
        },
        Keys {
            left: Semicolon,
            right: Backslash,
            up: LeftBracket,
            down: Apostrophe,
        },
        Keys {
            left: A,
            right: D,
            up: W,
            down: S,
        },
        Keys {
            left: F,
            right: H,
            up: T,
            down: G,
        },
        Keys {
            left: J,
            right: L,
            up: I,
            down: K,
        },
        Keys {
            left: Semicolon,
            right: Backslash,
            up: LeftBracket,
            down: Apostrophe,
        },
    ];
    let mut colors = Vec::new();
    colors.push(Color::RGB(255, 0, 0));
    colors.push(Color::RGB(0, 255, 0));
    colors.push(Color::RGB(0, 0, 255));
    colors.push(Color::RGB(0, 255, 255));
    colors.push(Color::RGB(255, 0, 255));
    colors.push(Color::RGB(255, 255, 0));
    colors.push(Color::RGB(255, 255, 255));
    colors.push(Color::RGB(128, 128, 128));

    'restart: loop {
        let mut cursors = Vec::new();
        let mut positions = Vec::new();
        let mut velocities = Vec::new();
        let mut owners = Vec::new();
        let mut buckets = Vec::new();
        buckets.resize(
            (BUCKETS_PER_ROW * BUCKETS_PER_COLUMN) as usize,
            SmallVec::<[Id; 12]>::new(),
        );

        let spacing = BALL_DIAMETER as usize;
        let start_positions = [
            [0, 0],
            [1, 0],
            [2, 0],
            [3, 0],
            [0, 1],
            [1, 1],
            [2, 1],
            [3, 1],
        ];
        let mut id_counter = 0;
        for (player_id, start_pos) in start_positions.iter().enumerate() {
            for x in (0..(LEVEL_WIDTH / 6)).step_by(spacing) {
                for y in (0..(LEVEL_HEIGHT / 4)).step_by(spacing) {
                    let pos = Vec2::new(
                        (start_pos[0] as f32) * (LEVEL_WIDTH / 4) as f32 + x as f32,
                        (start_pos[1] as f32) * (LEVEL_HEIGHT / 2) as f32 + y as f32,
                    );
                    if cursors.len() == player_id {
                        cursors.push(Vec2::new(pos.x + 150., pos.y + 150.));
                    }
                    positions.push(pos);
                    let bucket_pos = pos.as_ivec2() / BUCKET_SIZE;
                    buckets[get_bucket_index(bucket_pos)].push(id_counter);
                    id_counter += 1;
                    velocities.push(Vec2::ZERO);
                    owners.push(player_id);
                }
            }
        }
        dbg!(positions.len());
        dbg!(positions.len() / cursors.len());

        let mut start = Instant::now();
        'main_loop: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            for (i, &pos) in positions.iter().enumerate() {
                velocities[i].x += (cursors[owners[i]].x - pos.x) * CURSOR_FOLLOW_SPEED;
                velocities[i].y += (cursors[owners[i]].y - pos.y) * CURSOR_FOLLOW_SPEED;
            }

            for (i, pos) in positions.iter_mut().enumerate() {
                let new_pos = *pos + velocities[i];
                velocities[i] *= VELOCITY_SLOWDOWN;
                let old_bucket_pos = pos.as_ivec2() / BUCKET_SIZE;
                let new_bucket_pos = new_pos.as_ivec2() / BUCKET_SIZE;
                if new_bucket_pos.x < 0
                    || new_bucket_pos.x >= BUCKETS_PER_ROW
                    || new_bucket_pos.y < 0
                    || new_bucket_pos.y >= BUCKETS_PER_COLUMN
                {
                    continue;
                }
                if old_bucket_pos != new_bucket_pos {
                    let old_bucket = &mut buckets[get_bucket_index(old_bucket_pos)];
                    let index = old_bucket.iter().position(|id| *id == i as Id).unwrap();
                    old_bucket.remove(index);
                    let new_bucket = &mut buckets[get_bucket_index(new_bucket_pos)];
                    new_bucket.push(i as Id);
                }
                *pos = new_pos;
            }

            for (a_id, a) in positions.iter().enumerate() {
                let bucket_pos = a.as_ivec2() / BUCKET_SIZE;
                for bucket_x in
                    (bucket_pos.x - 1).max(0)..=(bucket_pos.x + 1).min(BUCKETS_PER_ROW - 1)
                {
                    for bucket_y in
                        (bucket_pos.y - 1).max(0)..=(bucket_pos.y + 1).min(BUCKETS_PER_COLUMN - 1)
                    {
                        for &b_id in &buckets[get_bucket_index(IVec2 {
                            x: bucket_x,
                            y: bucket_y,
                        })] {
                            if a_id == b_id as usize {
                                continue;
                            }
                            let b = positions[b_id as usize];
                            let dx = b.x - a.x;
                            let dy = b.y - a.y;
                            if (dx * dx + dy * dy) < BALL_DIAMETER * BALL_DIAMETER {
                                let evasion_speed = if owners[a_id] != owners[b_id as usize] {
                                    ENEMY_EVASION
                                } else {
                                    FRIENDLY_EVASION
                                };
                                let dx = (b.x - a.x) * evasion_speed;
                                let dy = (b.y - a.y) * evasion_speed;
                                velocities[a_id].x -= dx;
                                velocities[a_id].y -= dy;
                                velocities[b_id as usize].x += dx;
                                velocities[b_id as usize].y += dy;
                            }
                        }
                    }
                }
            }

            for (id, pos) in cursors.iter().enumerate() {
                canvas.set_draw_color(colors[id]);
                _ = canvas.draw_rect(Rect::new(
                    pos.x as i32,
                    pos.y as i32,
                    (BALL_DIAMETER * 2.) as u32,
                    (BALL_DIAMETER * 2.) as u32,
                ));
            }
            for (i, pos) in positions.iter().enumerate() {
                canvas.set_draw_color(colors[owners[i]]);
                _ = canvas.fill_rect(Rect::new(
                    pos.x as i32,
                    pos.y as i32,
                    VISUAL_DIAMETER,
                    VISUAL_DIAMETER,
                ));
            }

            let frame_duration = start.elapsed().as_secs_f32() * 1000.;
            start = Instant::now();
            // dbg!((1000. / frame_duration) as u32);
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            _ = canvas.fill_rect(Rect::new(0, 0, (1000. / frame_duration) as u32, 10));
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            _ = canvas.draw_rect(Rect::new(0, 0, 200, 10));
            _ = canvas.draw_rect(Rect::new(0, 0, 400, 10));

            canvas.present();
            let keyboard = event_pump.keyboard_state();
            for (id, keys) in player_keys.iter().enumerate() {
                if keyboard.is_scancode_pressed(keys.left) {
                    cursors[id].x -= CURSOR_SPEED;
                }
                if keyboard.is_scancode_pressed(keys.right) {
                    cursors[id].x += CURSOR_SPEED;
                }
                if keyboard.is_scancode_pressed(keys.up) {
                    cursors[id].y -= CURSOR_SPEED;
                }
                if keyboard.is_scancode_pressed(keys.down) {
                    cursors[id].y += CURSOR_SPEED;
                }
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'restart,
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => break 'main_loop,
                    _ => {}
                }
            }

            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        }
    }
}

fn get_bucket_index(bucket_pos: IVec2) -> usize {
    (bucket_pos.x + bucket_pos.y * BUCKETS_PER_ROW) as usize
}
