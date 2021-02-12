use sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

pub mod physics;
use physics::ball_physics;

fn main() {
	//settings
	let scale: u32 = 20;
	let width = 800;
	let height = 600;
	let center_x = width/2;
	let center_y = height/2;
	let dotsize: u32 = 5;
	//rendering code
	let mut physics_instance = ball_physics::BallPhysics::new();
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
    	//graphics code
        canvas.set_draw_color(Color::RGB(16,32,64));
        canvas.clear();
        //grid drawing fucntion
        let map = physics_instance.get_sectors();
        for y in -10..11{
        	for x in -10..11{
        		if map.contains_key(&(x,y-1)) {
        			canvas.set_draw_color(Color::RGB(64,128,64));
        		}
        		else{
        			canvas.set_draw_color(Color::RGB(64,64,64));
        		}
        		let tile_coords = Rect::new(
        			center_x + x * scale as i32, 
        			center_y - y * scale as i32, 
        			scale-1, 
        			scale-1
        			);
        		canvas.fill_rect(tile_coords).ok().unwrap();
        	}
        }
    	//draw balls to canvas
    	for (_id, ball) in physics_instance.get_balls() {
    		let x = ball.pos.x;
    		let y = ball.pos.y;
        	let dot_coords = Rect::new(
        		center_x + (x * scale as f32) as i32 - dotsize as i32, 
        		center_y - (y * scale as f32) as i32 - dotsize as i32, 
        		dotsize*2, 
        		dotsize*2
        	);
    		canvas.set_draw_color(Color::RGB(192,54,54));
    		canvas.fill_rect(dot_coords).ok().unwrap();
    	}
        //event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                	println!("Closing!");
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        physics_instance.update(1.0f32/60.0f32);
    	println!("frame end");
    }
    println!("Bye!");
}
