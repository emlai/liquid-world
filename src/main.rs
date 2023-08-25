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
#[derive(Copy, Clone)]
struct Pos {
    pos: Vec2,
    id: usize, // needed anymore?
}

const VISUAL_DIAMETER: u32 = 6;
const BALL_DIAMETER: f32 = 8.;
const BUCKET_SIZE: i32 = BALL_DIAMETER as i32;
const BUCKETS_PER_ROW: i32 = LEVEL_WIDTH / BUCKET_SIZE;
const BUCKETS_PER_COLUMN: i32 = LEVEL_HEIGHT / BUCKET_SIZE;
const LEVEL_WIDTH: i32 = 1600;
const LEVEL_HEIGHT: i32 = 896; // to fit even number of buckets

fn get_bucket_index(bucket_pos: IVec2) -> usize {
    (bucket_pos.x + bucket_pos.y * BUCKETS_PER_ROW) as usize
}

pub fn main() {
    dbg!(BUCKET_SIZE, BUCKETS_PER_COLUMN, BUCKETS_PER_ROW);
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
        for _ in 0..(BUCKETS_PER_ROW * BUCKETS_PER_COLUMN) {
            buckets.push(SmallVec::<[usize; 16]>::new()); // Make this smallvec?
        }

        let spacing = 10;
        let mut counter = 0;
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
        for (player_id, start_pos) in start_positions.iter().enumerate() {
            for x in (0..(LEVEL_WIDTH / 5)).step_by(spacing) {
                for y in (0..(LEVEL_HEIGHT / 3)).step_by(spacing) {
                    let pos = Pos {
                        pos: Vec2::new(
                            (start_pos[0] as f32) * (LEVEL_WIDTH / 4) as f32 + x as f32,
                            (start_pos[1] as f32) * (LEVEL_HEIGHT / 2) as f32 + y as f32,
                        ),
                        id: counter,
                    };
                    if cursors.len() == player_id {
                        cursors.push(Vec2::new(pos.pos.x + 200., pos.pos.y + 200.));
                    }
                    positions.push(pos);
                    let bucket_pos = pos.pos.as_ivec2() / BUCKET_SIZE;
                    buckets[get_bucket_index(bucket_pos)].push(pos.id);
                    counter += 1;
                    velocities.push(Vec2 { x: 0., y: 0. });
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
            for pos in &positions {
                let cursor_follow_speed = 0.001f32;
                velocities[pos.id].x +=
                    (cursors[owners[pos.id]].x - pos.pos.x) * cursor_follow_speed;
                velocities[pos.id].y +=
                    (cursors[owners[pos.id]].y - pos.pos.y) * cursor_follow_speed;
            }

            for (i, pos) in positions.iter_mut().enumerate() {
                // let _actual_bucket = buckets.iter().position(|b| b.contains(&pos.id));
                let new_pos = pos.pos + velocities[pos.id];
                velocities[pos.id] *= 0.9;
                let old_bucket_pos = pos.pos.as_ivec2() / BUCKET_SIZE;
                let new_bucket_pos = new_pos.as_ivec2() / BUCKET_SIZE;
                if new_bucket_pos.x < 0
                    || new_bucket_pos.x >= BUCKETS_PER_ROW
                    || new_bucket_pos.y < 0
                    || new_bucket_pos.y >= BUCKETS_PER_COLUMN
                {
                    continue;
                }
                if old_bucket_pos != new_bucket_pos {
                    // faster with or without this `if`?
                    let old_bucket = &mut buckets[get_bucket_index(old_bucket_pos)];
                    let index = old_bucket.iter().position(|id| *id == pos.id).unwrap();
                    old_bucket.remove(index);
                    let new_bucket = &mut buckets[get_bucket_index(new_bucket_pos)];
                    new_bucket.push(pos.id);
                }
                pos.pos = new_pos;
                // update_pos(positions, i, );
            }

            for a in &positions {
                let bucket_pos = a.pos.as_ivec2() / BUCKET_SIZE;
                for bucket_x in (bucket_pos.x - 1)..=(bucket_pos.x + 1) {
                    for bucket_y in (bucket_pos.y - 1)..=(bucket_pos.y + 1) {
                        if bucket_x < 0
                            || bucket_x >= BUCKETS_PER_ROW
                            || bucket_y < 0
                            || bucket_y >= BUCKETS_PER_COLUMN
                        {
                            continue;
                        }
                        for &b_id in &buckets[get_bucket_index(IVec2 {
                            x: bucket_x,
                            y: bucket_y,
                        })] {
                            if a.id == b_id {
                                continue;
                            }
                            let b_pos = positions[b_id].pos;
                            let dx = b_pos.x - a.pos.x;
                            let dy = b_pos.y - a.pos.y;
                            if (dx * dx + dy * dy) < BALL_DIAMETER * BALL_DIAMETER {
                                let evasion_speed = if owners[a.id] != owners[b_id] {
                                    0.1f32
                                } else {
                                    0.05f32
                                };
                                let dx = (b_pos.x - a.pos.x) * evasion_speed;
                                let dy = (b_pos.y - a.pos.y) * evasion_speed;
                                velocities[a.id].x -= dx;
                                velocities[a.id].y -= dy;
                                velocities[b_id].x += dx;
                                velocities[b_id].y += dy;
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
            for pos in positions.iter() {
                canvas.set_draw_color(colors[owners[pos.id]]);
                _ = canvas.fill_rect(Rect::new(
                    pos.pos.x as i32,
                    pos.pos.y as i32,
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

            let spilled_count = buckets.iter().filter(|b| b.spilled()).count();
            if spilled_count > 0 {
                println!("{}", spilled_count);
            }

            canvas.present();
            let keyboard = event_pump.keyboard_state();
            let cursor_speed = 5f32;
            for (id, keys) in player_keys.iter().enumerate() {
                if keyboard.is_scancode_pressed(keys.left) {
                    cursors[id].x -= cursor_speed;
                }
                if keyboard.is_scancode_pressed(keys.right) {
                    cursors[id].x += cursor_speed;
                }
                if keyboard.is_scancode_pressed(keys.up) {
                    cursors[id].y -= cursor_speed;
                }
                if keyboard.is_scancode_pressed(keys.down) {
                    cursors[id].y += cursor_speed;
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
