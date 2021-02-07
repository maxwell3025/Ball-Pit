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
		out.add_ball(Ball::new(0.5f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32));
		out
	}

	pub fn update(&mut self, dt: f32){
		//TODO create update/interaction function
		self.clean();
		self.sectorize();
		self.update_collisions();
		//check for connections
		//apply forces
	}

	//clean up data structures
	pub fn clean(&mut self){
		self.sectors.clear();
		self.connections.clear();
	}

	//add balls into sectors
	pub fn sectorize(&mut self){
		for (id, ball) in &self.balls{
			let x = ball.pos.x;
			let y = ball.pos.y;
			let rad = ball.rad;
			let x = x.floor() as i32;
			let y = y.floor() as i32;
			let rad = rad.ceil() as i32;

			for sector_y in y-rad .. y+rad+1{
				for sector_x in x-rad .. x+rad+1{
					if !self.sectors.contains_key(&(sector_x, sector_y)) {
						self.sectors.insert((sector_x, sector_y), Vec::new());
					}
					self.sectors.get_mut(&(sector_x, sector_y)).unwrap().push(*id);
				}
			}

		}
	}

	pub fn update_collisions(&mut self){
		for (_sector,id_list) in &self.sectors {
			for i in 0..id_list.len(){
				for j in 0..i{
					let ball_a = self.balls.get(&id_list[i]).unwrap();
					let ball_b = self.balls.get(&id_list[j]).unwrap();
					let diff = ball_a.pos - ball_b.pos;
					let req_dist = ball_a.rad + ball_b.rad;
					if diff.magnitude() <= req_dist{
						let mut pair = (id_list[i], id_list[j]);
						if pair.0 > pair.1 {
							std::mem::swap(&mut pair.0, &mut pair.1);
						}
						self.connections.insert(pair);
						println!("connected nodes {} and {}", pair.0, pair.1);
					}
				}
			}
		}
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