use nalgebra::Vector2;

pub struct Ball{
	pub pos: Vector2<f32>,
	pub vel: Vector2<f32>,
	pub rad: f32,
}

impl Ball{
	//constructor
	fn new(x_pos: f32, y_pos:f32, x_vel:f32, y_vel:f32, rad:f32) -> Ball{
		let pos = Vector2::new(x_pos, y_pos);
		let vel = Vector2::new(x_vel, y_vel);
		Ball{pos, vel, rad}
	}
	fn stationary(x_pos: f32, y_pos:f32, rad:f32) -> Ball{
		Ball::new(x_pos, y_pos, 0.0f32, 0.0f32, rad)
	}
	//physics update function
	fn update(&mut self, dt: f32){
		self.pos += self.vel*dt;
	}
}