use nalgebra::Vector2;

pub struct Ball{
	pos: Vector2<f32>,
	vel: Vector2<f32>,
	rad: f32,
}

impl Ball{
	fn new(x: f32, y:f32, rad:f32) -> Ball{
		let pos = Vector2::new(x, y);
		let vel = Vector2::new(0.0f32, 0.0f32);
		Ball{pos, vel, rad}
	}
	fn update(&mut self, dt: f32){
		self.pos += self.vel*dt;
	}
}