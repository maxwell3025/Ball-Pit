 use std::collections::HashMap;
use nalgebra::Vector2;

use super::ball::Ball;

pub struct BallPhysics{
	balls: Vec<Ball>,
	sectors: HashMap<Vector2<i32>, Vec<u32>>,
}
impl BallPhysics{
	//constructor
	pub fn new() -> BallPhysics{
		let balls = Vec::new();
		let sectors = HashMap::new();
		BallPhysics{balls, sectors}
	}

	pub fn update(&mut self, dt: f32){
		//add balls into sectors
		//check for connections
		//apply forces
	}
}