use nalgebra::Vector2;

pub struct Wall{
	pub a: Vector2<f32>,
	pub b: Vector2<f32>,
}

impl Wall{
	//constructor
	pub fn new(a_x: f32, a_y: f32, b_x: f32, b_y: f32) -> Wall{
		let a = Vector2::new(a_x, a_y);
		let b = Vector2::new(b_x, b_y);
		Wall{a, b}
	}

	//finds closest point on wall
	pub fn closest(&self, point: Vector2<f32>) -> Vector2<f32>{
		let wall_vector = self.a-self.b;
		if (point-self.a).dot(&wall_vector) > 0.0f32{
			return self.a;
		}
		if (point-self.b).dot(&wall_vector) < 0.0f32{
			return self.b;
		}
		let relative_position = point-self.b;
		let relative_position = relative_position.dot(&wall_vector) / wall_vector.norm_squared() * wall_vector;
		relative_position + self.b
	}
}