use std::collections::HashMap;
use std::collections::HashSet;
use rand::thread_rng;
use rand::Rng;
use nalgebra::Vector2;

use super::ball::Ball;
use super::wall;
use super::interaction;

pub const GRAVITY: f32 = 0.0;
pub const WALL_SIZE: f32 = 32.0;
pub const WALL_SEGS: i32 = 32;
pub const BALL_COUNT: i32 = 64;
pub const TEMP: f32 = 1000.0;

pub struct BallPhysics {
    balls: Vec<Ball>,
    walls: Vec<wall::Wall>,
    sectors: HashMap<(i32, i32), Vec<usize>>,
    connections: HashSet<(usize, usize)>,
    current_index: i64,
}

impl BallPhysics {
    //constructor
    pub fn new() -> BallPhysics {
        let balls = Vec::new();
        let walls = Vec::new();
        let sectors = HashMap::new();
        let connections = HashSet::new();
        let current_index = 0;
        let mut out = BallPhysics { balls, walls, sectors, connections, current_index };
        //generate gas
        let mut rng = thread_rng();
        let vel = (TEMP * 2.).sqrt();
        for _ in 0..BALL_COUNT {
            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.);
            out.add_ball(Ball::polar().
                with_pos(
                    rng.gen_range(-WALL_SIZE + 1.0f32..WALL_SIZE - 1.0f32),
                    rng.gen_range(-WALL_SIZE + 1.0f32..WALL_SIZE - 1.0f32),
                ).
                with_vel(
                    vel * angle.cos(),
                    vel * angle.sin(),
                )
            );
        }
        //generate walls
        for wall_position in 0..WALL_SEGS {
            let lerp_value = (wall_position as f32) / (WALL_SEGS as f32);
            let segment_size = WALL_SIZE / WALL_SEGS as f32;
            out.add_ball(Ball::blank().with_pos(WALL_SIZE * 2.0f32 * lerp_value - WALL_SIZE, -WALL_SIZE).with_rad(segment_size).with_mass(1.0e20f32));
            out.add_ball(Ball::blank().with_pos(WALL_SIZE - WALL_SIZE * 2.0f32 * lerp_value, WALL_SIZE).with_rad(segment_size).with_mass(1.0e20f32));
            out.add_ball(Ball::blank().with_pos(WALL_SIZE, WALL_SIZE * 2.0f32 * lerp_value - WALL_SIZE).with_rad(segment_size).with_mass(1.0e20f32));
            out.add_ball(Ball::blank().with_pos(-WALL_SIZE, WALL_SIZE - WALL_SIZE * 2.0f32 * lerp_value).with_rad(segment_size).with_mass(1.0e20f32));
        }
        out
    }

    pub fn update(&mut self, dt: f32) {
        //TODO create update/interaction function
        self.clean();
        self.sectorize();
        self.update_contacts();
        self.do_physics(dt);
        //check for connections
        //apply forces
    }

    //clean up data structures
    pub fn clean(&mut self) {
        self.sectors.clear();
        self.connections.clear();
    }

    //add balls into sectors
    fn sectorize(&mut self) {
        for (id, ball) in self.balls.iter().enumerate() {
            let x = ball.pos.x;
            let y = ball.pos.y;
            let range = ball.range;
            let x = x.floor() as i32;
            let y = y.floor() as i32;
            let range = range.ceil() as i32;

            for sector_y in y - range..y + range + 1 {
                for sector_x in x - range..x + range + 1 {
                    if !self.sectors.contains_key(&(sector_x, sector_y)) {
                        self.sectors.insert((sector_x, sector_y), Vec::new());
                    }
                    self.sectors.get_mut(&(sector_x, sector_y)).unwrap().push(id);
                }
            }
        }
    }

    fn update_contacts(&mut self) {
        for (_sector, id_list) in &self.sectors {
            for i in 0..id_list.len() {
                'inner: for j in 0..i {
                    if self.connections.contains(&(id_list[i], id_list[j])) {
                        //println!("already have nodes {} and {}", id_list[i], id_list[j]);
                        continue 'inner;
                    }
                    let mut pair = (id_list[i], id_list[j]);
                    if pair.0 > pair.1 {
                        std::mem::swap(&mut pair.0, &mut pair.1);
                    }
                    self.connections.insert(pair);

                }
            }
        }
    }

    fn do_physics(&mut self, dt: f32) {

        //iterate through pairs in contact
        for (a, b) in &self.connections {
            let ball_a: &mut Ball;
            let ball_b: &mut Ball;
            unsafe {
                ball_a = &mut *(self.balls.get_unchecked_mut(*a) as *mut _);
                ball_b = &mut *(self.balls.get_unchecked_mut(*b) as *mut _);
            }
            let diff = ball_a.pos - ball_b.pos;
            let req_dist = ball_a.rad + ball_b.rad;
            if diff.magnitude() <= req_dist {
                BallPhysics::do_collision(ball_a, ball_b);
            }
            ball_a.force += interaction::get_force(ball_a, ball_b);
            ball_b.force += interaction::get_force(ball_b, ball_a);
        }

        //gravity
        for ball in &mut self.balls {
            ball.force += Vector2::new(0.0f32, GRAVITY);
        }

        //update balls
        for ball in &mut self.balls {
            ball.update(dt);
        }
    }
    //assuming that a and b are in contact
    fn do_collision(a: &mut Ball, b: &mut Ball) {
        //check to make sure infinite collision loops dont happen by making sure balls are headed towards each other
        let diff:Vector2<f32> = a.pos - b.pos;
        if diff.dot(&(a.vel - b.vel)) > 0.0f32 {
            return;
        }

        //find projections
        let proj_factor = 1.0f32 / diff.norm_squared();
        let a_vel = a.vel.dot(&diff) * proj_factor * &diff;
        let b_vel = b.vel.dot(&diff) * proj_factor * &diff;

        //solve collisions
        let momentum_transfer = 2.0f32 / (a.mass + b.mass) * (b_vel - a_vel);
        a.vel += momentum_transfer * b.mass;
        b.vel -= momentum_transfer * a.mass;
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
        self.current_index += 1;
        //TODO check for collisions
    }

    pub fn get_balls(&self) -> &Vec<Ball> {
        &self.balls
    }

    pub fn get_sectors(&self) -> &HashMap<(i32, i32), Vec<usize>> {
        &self.sectors
    }
}