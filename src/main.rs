extern crate rstar;
extern crate sdl2;

use rstar::{RTree, RTreeObject, AABB};
use sdl2::event::Event;
use sdl2::keyboard::Scancode::*;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

struct Keys {
    left: Scancode,
    right: Scancode,
    up: Scancode,
    down: Scancode,
}
struct Pos {
    x: f32,
    y: f32,
}
#[derive(Copy, Clone)]
struct RTreePos {
    x: f32,
    y: f32,
    id: usize,
}
impl RTreeObject for RTreePos {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x, self.y])
    }
}

const BALL_DIAMETER: f32 = 8.;
const VISUAL_DIAMETER: u32 = 6;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_width = 1600;
    let window_height = 900;

    let window = video_subsystem
        .window("rust-sdl2 demo", window_width, window_height)
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
        let mut positions = RTree::<RTreePos>::new();
        let mut velocities = Vec::new();
        let mut owners = Vec::new();
        // let mut collisions = Vec::new();

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
            for x in (0..(window_width / 5)).step_by(spacing) {
                for y in (0..(window_height / 3)).step_by(spacing) {
                    let pos = RTreePos {
                        x: (start_pos[0] as f32) * (window_width / 4) as f32 + x as f32,
                        y: (start_pos[1] as f32) * (window_height / 2) as f32 + y as f32,
                        id: counter,
                    };
                    if cursors.len() == player_id {
                        cursors.push(Pos { x: pos.x, y: pos.y });
                    }
                    positions.insert(pos);
                    counter += 1;
                    velocities.push(Pos { x: 0., y: 0. });
                    owners.push(player_id);
                }
            }
        }
        dbg!(positions.size());

        'main_loop: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            for pos in positions.iter() {
                let cursor_follow_speed = 0.001f32;
                velocities[pos.id].x += (cursors[owners[pos.id]].x - pos.x) * cursor_follow_speed;
                velocities[pos.id].y += (cursors[owners[pos.id]].y - pos.y) * cursor_follow_speed;
            }

            positions = RTree::bulk_load(
                positions
                    .iter()
                    .map(|p| {
                        let mut pos = *p;
                        pos.x += velocities[pos.id].x;
                        pos.y += velocities[pos.id].y;
                        velocities[pos.id].x *= 0.9;
                        velocities[pos.id].y *= 0.9;
                        pos
                    })
                    .collect(),
            );

            let mut counter = 0;
            for a in positions.iter() {
                for b in positions.locate_in_envelope(&AABB::from_corners(
                    [a.x - BALL_DIAMETER, a.y - BALL_DIAMETER],
                    [a.x + BALL_DIAMETER, a.y + BALL_DIAMETER],
                )) {
                    counter += 1;
                    if a.id == b.id {
                        continue;
                    }
                    let dx = b.x - a.x;
                    let dy = b.y - a.y;
                    if (dx * dx + dy * dy) < BALL_DIAMETER * BALL_DIAMETER {
                        let evasion_speed = if owners[a.id] != owners[b.id] {
                            0.1f32
                        } else {
                            0.01f32
                        };
                        let dx = (b.x - a.x) * evasion_speed;
                        let dy = (b.y - a.y) * evasion_speed;
                        velocities[a.id].x -= dx;
                        velocities[a.id].y -= dy;
                        velocities[b.id].x += dx;
                        velocities[b.id].y += dy;
                    }
                }
            }
            // dbg!(counter);

            for (id, pos) in cursors.iter().enumerate() {
                canvas.set_draw_color(colors[owners[id]]);
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
                    pos.x as i32,
                    pos.y as i32,
                    VISUAL_DIAMETER,
                    VISUAL_DIAMETER,
                ));
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

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        }
    }
}
