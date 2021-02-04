use super::ball::Ball;

pub struct BallPhysics{
	balls: Vec<Ball>
}
impl BallPhysics{
	//constructor
	pub fn new() -> BallPhysics{
		let balls = Vec::new();
		BallPhysics{balls}
	}

	pub fn update(&mut self, dt: f32){
		
	}
}