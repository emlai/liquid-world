extern crate sdl2;

use aabb_quadtree::QuadTree;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::ptr;
use std::time::Duration;

struct Pos {
    x: f32,
    y: f32,
}
const BALL_DIAMETER: f32 = 10.;
const VISUAL_DIAMETER: u32 = 5;

struct Collision {
    a: usize,
    b: usize,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 1600, 900)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    let mut colors = Vec::new();

    let spacing = 15;
    for x in (0..1000).step_by(spacing) {
        for y in (0..1000).step_by(spacing) {
            positions.push(Pos {
                x: x as f32 + 100.,
                y: y as f32 + 300.,
            });
            velocities.push(Pos { x: 0., y: 0. });
            colors.push(Color::RGB(255, 0, 0));
        }
    }
    for x in (0..1000).step_by(spacing) {
        for y in (0..1000).step_by(spacing) {
            positions.push(Pos {
                x: x as f32 + 900.,
                y: y as f32 + 300.,
            });
            velocities.push(Pos { x: 0., y: 0. });
            colors.push(Color::RGB(0, 255, 0));
        }
    }

    'main_loop: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let cursor_x = positions[0].x;
        let cursor_y = positions[0].y;
        for i in 1..positions.len() {
            let cursor_follow_speed = 0.001f32;
            velocities[i].x += (cursor_x - positions[i].x) * cursor_follow_speed;
            velocities[i].y += (cursor_y - positions[i].y) * cursor_follow_speed;
        }

        for i in 0..positions.len() {
            positions[i].x += velocities[i].x;
            positions[i].y += velocities[i].y;
            velocities[i].x *= 0.9;
            velocities[i].y *= 0.9;
        }

        let mut collisions = Vec::new();
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let dx = positions[j].x - positions[i].x;
                let dy = positions[j].y - positions[i].y;
                if (dx * dx + dy * dy) < BALL_DIAMETER * BALL_DIAMETER {
                    collisions.push(Collision { a: i, b: j });
                }
            }
        }

        for collision in &collisions {
            let evasion_speed = 0.01f32;
            let dx = (positions[collision.b].x - positions[collision.a].x) * evasion_speed;
            let dy = (positions[collision.b].y - positions[collision.a].y) * evasion_speed;
            velocities[collision.a].x -= dx;
            velocities[collision.a].y -= dy;
            velocities[collision.b].x += dx;
            velocities[collision.b].y += dy;
        }
        for i in 0..positions.len() {
            canvas.set_draw_color(colors[i]);
            _ = canvas.fill_rect(Rect::new(
                positions[i].x as i32,
                positions[i].y as i32,
                VISUAL_DIAMETER,
                VISUAL_DIAMETER,
            ));
        }

        canvas.present();
        let keyboard = event_pump.keyboard_state();
        let speed = 0.5f32;
        if keyboard.is_scancode_pressed(Scancode::Left) {
            velocities[0].x -= speed;
        }
        if keyboard.is_scancode_pressed(Scancode::Right) {
            velocities[0].x += speed;
        }
        if keyboard.is_scancode_pressed(Scancode::Up) {
            velocities[0].y -= speed;
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            velocities[0].y += speed;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}
