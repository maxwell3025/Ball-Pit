use std::collections::HashMap; 
use std::collections::HashSet;

use super::ball::Ball;

pub struct BallPhysics{
	balls: HashMap<i64, Ball>,
	sectors: HashMap<(i32,i32), Vec<i64>>,
	connections: HashSet<(i64,i64)>,
	current_index: i64,
}
impl BallPhysics{
	//constructor
	pub fn new() -> BallPhysics{
		let balls = HashMap::new();
		let sectors = HashMap::new();
		let connections = HashSet::new();
		let current_index = 0;
		let mut out = BallPhysics{balls, sectors, connections, current_index};
		out.add_ball(Ball::new(0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32));
		out
	}

	pub fn update(&mut self, dt: f32){
		//TODO create update/interaction function
		//add balls into sectors
		for (id, ball) in &self.balls{
			let x = ball.pos.x;
			let y = ball.pos.y;
			let x = x.floor() as i32;
			let y = y.floor() as i32;

		}
		//check for connections
		//apply forces
		//clean up data structures
		self.connections.clear();
	}

	pub fn add_ball(&mut self, ball: Ball){
		self.balls.insert(self.current_index, ball);
		self.current_index += 1;
	}

	pub fn get_balls(&self) -> &HashMap<i64, Ball>{
		&self.balls
	}

	pub fn get_sectors(&self) -> &HashMap<(i32,i32), Vec<i64>>{
		&self.sectors
	}
}