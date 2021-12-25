use sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::video::*;
use std::time::Duration;

pub mod physics;

use physics::ball_physics;

fn main() {
    //settings
    let scale: u32 = 8;
    let width: i32 = 512;
    let height: i32 = 512;
    let center_x: i32 = width / 2;
    let center_y: i32 = height / 2;
    let mut t: f32 = 0.0;
    //rendering code
    let mut physics_instance = ball_physics::BallPhysics::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Ball Pit", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        //graphics code
        canvas.set_draw_color(Color::RGB(16, 32, 64));
        canvas.clear();
        //grid drawing fucntion
        let map = physics_instance.get_sectors();
        for y in -16..16 {
            for x in -16..16 {
                if map.contains_key(&(x, y)) {
                    canvas.set_draw_color(Color::RGB(64, 128, 64));
                } else {
                    canvas.set_draw_color(Color::RGB(64, 64, 64));
                }
                let tile_coords = Rect::new(
                    center_x + x * scale as i32,
                    center_y - (y + 1) * scale as i32,
                    scale - 1,
                    scale - 1,
                );
                canvas.fill_rect(tile_coords).ok().unwrap();
            }
        }
        //draw fields to canvas
        for ball in physics_instance.get_balls() {
            let x = ball.pos.x;
            let y = ball.pos.y;
            let dot_coords = (
                center_x + (x * scale as f32) as i32,
                center_y - (y * scale as f32) as i32
            );
            canvas.set_draw_color(Color::RGBA(54, 54, 128,128));
            fill_circle(&mut canvas, dot_coords.0, dot_coords.1, ball.range * scale as f32);
        }
        //draw balls to canvas
        for ball in physics_instance.get_balls() {
            let x = ball.pos.x;
            let y = ball.pos.y;
            let dot_coords = (
                center_x + (x * scale as f32) as i32,
                center_y - (y * scale as f32) as i32
            );
            canvas.set_draw_color(Color::RGB(192, 54, 54));
            fill_circle(&mut canvas, dot_coords.0, dot_coords.1, ball.rad * scale as f32);
        }
        //event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Closing!");
                    break 'running;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        physics_instance.update(1.0f32 / 120.0f32);
        t += 1.0f32 / 120.0f32;
        println!("t = {}", t);
    }
    println!("Bye!");
}

fn fill_circle(canvas: &mut Canvas<Window>, x: i32, y: i32, rad: f32) {
    let pixel_width = rad.floor() as i32;
    for n in -pixel_width..pixel_width + 1 {
        let height = (rad * rad - (n * n) as f32).sqrt().floor() as i32;
        let strip = Rect::new(
            x + n,
            y - height,
            1,
            (height * 2 + 1) as u32,
        );
        canvas.fill_rect(strip).ok().unwrap();
    }
}
