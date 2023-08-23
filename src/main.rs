extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::ptr;
use std::time::Duration;

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
}
const BALL_DIAMETER: f32 = 20.;

// fn distance(a: &Ball, b: &Ball) -> f32 {
// }

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

    let mut balls = Vec::new();
    let spacing = 30;
    for x in (0..500).step_by(spacing) {
        for y in (0..500).step_by(spacing) {
            balls.push(Ball {
                x: x as f32 + 100.,
                y: y as f32 + 400.,
                vx: 0.,
                vy: 0.,
                color: Color::RGB(255, 0, 0),
            })
        }
    }
    for x in (0..500).step_by(spacing) {
        for y in (0..500).step_by(spacing) {
            balls.push(Ball {
                x: x as f32 + 900.,
                y: y as f32 + 400.,
                vx: 0.,
                vy: 0.,
                color: Color::RGB(0, 255, 0),
            })
        }
    }

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let cursor_x = balls[0].x;
        let cursor_y = balls[0].y;
        for ball in balls.iter_mut().skip(1) {
            let cursor_follow_speed = 0.001f32;
            ball.vx += (cursor_x - ball.x) * cursor_follow_speed;
            ball.vy += (cursor_y - ball.y) * cursor_follow_speed;
        }

        for ball in &mut balls {
            ball.x += ball.vx;
            ball.y += ball.vy;
            ball.vx *= 0.9;
            ball.vy *= 0.9;
        }

        let mut collisions = Vec::new();
        for (ball_index, ball) in balls.iter().enumerate() {
            for (other_ball_index, other_ball) in balls.iter().enumerate() {
                if ptr::eq(&other_ball, &ball) {
                    continue;
                }
                let dx = other_ball.x - ball.x;
                let dy = other_ball.y - ball.y;
                if (dx * dx + dy * dy) < BALL_DIAMETER * BALL_DIAMETER {
                    collisions.push(Collision {
                        a: ball_index,
                        b: other_ball_index,
                    });
                }
            }
        }

        for collision in &collisions {
            let a = &balls[collision.a];
            let b = &balls[collision.b];
            let evasion_speed = 0.01f32;
            let dx = (b.x - a.x) * evasion_speed;
            let dy = (b.y - a.y) * evasion_speed;
            {
                let a = &mut balls[collision.a];
                a.vx -= dx;
                a.vy -= dy;
            }
            {
                let b = &mut balls[collision.b];
                b.vx += dx;
                b.vy += dy;
            }
        }
        for ball in &balls {
            canvas.set_draw_color(ball.color);
            _ = canvas.fill_rect(Rect::new(ball.x as i32, ball.y as i32, 10, 10));
        }
        // _ = canvas.draw_points(
        //     &*balls
        //         .iter()
        //         .map(|ball| {
        //             Point::new(
        //                 ball.x as i32,
        //                 ball.y as i32,
        //                 // BALL_DIAMETER as u32,
        //                 // BALL_DIAMETER as u32,
        //             )
        //         })
        //         .collect::<Vec<_>>(),
        // );

        canvas.present();
        let keyboard = event_pump.keyboard_state();
        let speed = 0.5f32;
        if keyboard.is_scancode_pressed(Scancode::Left) {
            balls[0].vx -= speed;
        }
        if keyboard.is_scancode_pressed(Scancode::Right) {
            balls[0].vx += speed;
        }
        if keyboard.is_scancode_pressed(Scancode::Up) {
            balls[0].vy -= speed;
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            balls[0].vy += speed;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}
