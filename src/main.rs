extern crate raylib;
extern crate rstar;

use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDraw;
use rstar::{RTree, RTreeObject, AABB};
use std::time::Duration;

#[derive(Copy, Clone)]
struct Keys {
    left: KeyboardKey,
    right: KeyboardKey,
    up: KeyboardKey,
    down: KeyboardKey,
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
    let window_width = 1600;
    let window_height = 900;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Hello, World")
        .build();
    rl.set_exit_key(None);
    rl.set_target_fps(120);

    let player_keys = vec![
        Keys {
            left: KeyboardKey::KEY_A,
            right: KeyboardKey::KEY_D,
            up: KeyboardKey::KEY_W,
            down: KeyboardKey::KEY_S,
        },
        Keys {
            left: KeyboardKey::KEY_F,
            right: KeyboardKey::KEY_H,
            up: KeyboardKey::KEY_T,
            down: KeyboardKey::KEY_G,
        },
        Keys {
            left: KeyboardKey::KEY_J,
            right: KeyboardKey::KEY_L,
            up: KeyboardKey::KEY_I,
            down: KeyboardKey::KEY_K,
        },
        Keys {
            left: KeyboardKey::KEY_SEMICOLON,
            right: KeyboardKey::KEY_BACKSLASH,
            up: KeyboardKey::KEY_LEFT_BRACKET,
            down: KeyboardKey::KEY_APOSTROPHE,
        },
        Keys {
            left: KeyboardKey::KEY_A,
            right: KeyboardKey::KEY_D,
            up: KeyboardKey::KEY_W,
            down: KeyboardKey::KEY_S,
        },
        Keys {
            left: KeyboardKey::KEY_F,
            right: KeyboardKey::KEY_H,
            up: KeyboardKey::KEY_T,
            down: KeyboardKey::KEY_G,
        },
        Keys {
            left: KeyboardKey::KEY_J,
            right: KeyboardKey::KEY_L,
            up: KeyboardKey::KEY_I,
            down: KeyboardKey::KEY_K,
        },
        Keys {
            left: KeyboardKey::KEY_SEMICOLON,
            right: KeyboardKey::KEY_BACKSLASH,
            up: KeyboardKey::KEY_LEFT_BRACKET,
            down: KeyboardKey::KEY_APOSTROPHE,
        },
    ];
    let mut colors = Vec::new();
    colors.push(Color::new(255, 0, 0, 255));
    colors.push(Color::new(0, 255, 0, 255));
    colors.push(Color::new(0, 0, 255, 255));
    colors.push(Color::new(0, 255, 255, 255));
    colors.push(Color::new(255, 0, 255, 255));
    colors.push(Color::new(255, 255, 0, 255));
    colors.push(Color::new(255, 255, 255, 255));
    colors.push(Color::new(128, 128, 128, 255));

    'outer_loop: while !rl.window_should_close() {
        let mut cursors = Vec::new();
        let mut positions = RTree::<RTreePos>::new();
        let mut velocities = Vec::new();
        let mut owners = Vec::new();

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
            for x in (0..(window_width / 5)).step_by(10) {
                for y in (0..(window_height / 3)).step_by(10) {
                    let pos = RTreePos {
                        x: (start_pos[0] as f32) * (window_width / 4) as f32 + x as f32,
                        y: (start_pos[1] as f32) * (window_height / 2) as f32 + y as f32,
                        id: counter,
                    };
                    if cursors.len() == player_id {
                        cursors.push(Pos {
                            x: pos.x + 200.,
                            y: pos.y + 200.,
                        });
                    }
                    positions.insert(pos);
                    counter += 1;
                    velocities.push(Pos { x: 0., y: 0. });
                    owners.push(player_id);
                }
            }
        }
        dbg!(positions.size());
        dbg!(positions.size() / cursors.len());

        'main_loop: loop {
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

            for a in positions.iter() {
                for b in positions.locate_in_envelope(&AABB::from_corners(
                    [a.x - BALL_DIAMETER, a.y - BALL_DIAMETER],
                    [a.x + BALL_DIAMETER, a.y + BALL_DIAMETER],
                )) {
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

            {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_fps(0, 0);

                for (id, pos) in cursors.iter().enumerate() {
                    d.draw_circle_lines(pos.x as i32, pos.y as i32, BALL_DIAMETER, colors[id]);
                }
                for pos in positions.iter() {
                    d.draw_circle(
                        pos.x as i32,
                        pos.y as i32,
                        (VISUAL_DIAMETER / 2) as f32,
                        colors[owners[pos.id]],
                    );
                }
            }

            let cursor_speed = 5f32;
            for (id, keys) in player_keys.iter().enumerate() {
                if rl.is_key_down(keys.left) {
                    cursors[id].x -= cursor_speed;
                }
                if rl.is_key_down(keys.right) {
                    cursors[id].x += cursor_speed;
                }
                if rl.is_key_down(keys.up) {
                    cursors[id].y -= cursor_speed;
                }
                if rl.is_key_down(keys.down) {
                    cursors[id].y += cursor_speed;
                }
            }

            if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                break 'outer_loop;
            }
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                break 'main_loop;
            }

            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        }
    }
}
