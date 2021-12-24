use std::collections::HashMap;
use std::collections::HashSet;
use rand::thread_rng;
use rand::Rng;
use nalgebra::Vector2;

use super::ball;
use super::wall;

pub const GRAVITY: f32 = 0.0;
pub const WALL_SIZE: f32 = 32.0;
pub const WALL_SEGS: i32 = 32;
pub const BALL_COUNT: i32 = 10;

pub struct BallPhysics {
    balls: HashMap<i64, ball::Ball>,
    walls: HashMap<i64, wall::Wall>,
    sectors: HashMap<(i32, i32), Vec<i64>>,
    connections: HashSet<(i64, i64)>,
    current_index: i64,
}

impl BallPhysics {
    //constructor
    pub fn new() -> BallPhysics {
        let balls = HashMap::new();
        let walls = HashMap::new();
        let sectors = HashMap::new();
        let connections = HashSet::new();
        let current_index = 0;
        let mut out = BallPhysics { balls, walls, sectors, connections, current_index };
        //generate gas
        let mut rng = thread_rng();
        for _ in 0..BALL_COUNT {
            out.add_ball(ball::Ball::blank().
                with_pos(
                    rng.gen_range(-WALL_SIZE + 1.0f32..WALL_SIZE - 1.0f32),
                    rng.gen_range(-WALL_SIZE + 1.0f32..WALL_SIZE - 1.0f32),
                ).
                with_vel(
                    rng.gen_range(-WALL_SIZE + 1.0f32..WALL_SIZE - 1.0f32),
                    rng.gen_range(-WALL_SIZE + 1.0f32..WALL_SIZE - 1.0f32),
                )
            );
        }
        //generate walls
        for wall_position in 0..WALL_SEGS {
            let lerp_value = (wall_position as f32) / (WALL_SEGS as f32);
            let segment_size = WALL_SIZE / WALL_SEGS as f32;
            out.add_ball(ball::Ball::blank().with_pos(WALL_SIZE * 2.0f32 * lerp_value - WALL_SIZE, -WALL_SIZE).with_rad(segment_size).with_mass(1.0e10f32));
            out.add_ball(ball::Ball::blank().with_pos(WALL_SIZE - WALL_SIZE * 2.0f32 * lerp_value, WALL_SIZE).with_rad(segment_size).with_mass(1.0e10f32));
            out.add_ball(ball::Ball::blank().with_pos(WALL_SIZE, WALL_SIZE * 2.0f32 * lerp_value - WALL_SIZE).with_rad(segment_size).with_mass(1.0e10f32));
            out.add_ball(ball::Ball::blank().with_pos(-WALL_SIZE, WALL_SIZE - WALL_SIZE * 2.0f32 * lerp_value).with_rad(segment_size).with_mass(1.0e10f32));
        }
        out
    }

    pub fn update(&mut self, dt: f32) {
        //TODO create update/interaction function
        self.clean();
        self.sectorize();
        self.update_collisions();
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
        for (id, ball) in &self.balls {
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
                    self.sectors.get_mut(&(sector_x, sector_y)).unwrap().push(*id);
                }
            }
        }
    }

    fn update_collisions(&mut self) {
        for (_sector, id_list) in &self.sectors {
            for i in 0..id_list.len() {
                'inner: for j in 0..i {
                    if self.connections.contains(&(id_list[i], id_list[j])) {
                        //println!("already have nodes {} and {}", id_list[i], id_list[j]);
                        continue 'inner;
                    }
                    let ball_a = self.balls.get(&id_list[i]).unwrap();
                    let ball_b = self.balls.get(&id_list[j]).unwrap();
                    let diff = ball_a.pos - ball_b.pos;
                    let req_dist = ball_a.rad + ball_b.rad;
                    if diff.magnitude() <= req_dist {
                        let mut pair = (id_list[i], id_list[j]);
                        if pair.0 > pair.1 {
                            std::mem::swap(&mut pair.0, &mut pair.1);
                        }
                        self.connections.insert(pair);
                    }
                }
            }
        }
    }

    fn do_physics(&mut self, dt: f32) {
        //advect balls
        //generate key list -_-
        let mut keys = Vec::with_capacity(self.balls.len());
        for key in self.get_balls().keys() {
            keys.push(*key); //i hate it here i stg
        }

        //iterate through pairs in contact
        for (a, b) in &self.connections {
            let ball_a;
            let ball_b;
            unsafe {
                assert_ne!(a, b, "`a` ({:?}) must not equal `b` ({:?})", a, b);
                ball_a = &mut *(self.balls.get_mut(&a).unwrap() as *mut ball::Ball);
                ball_b = &mut *(self.balls.get_mut(&b).unwrap() as *mut ball::Ball);
            }
            BallPhysics::do_collision(ball_a, ball_b);
        }

        //iterate through individual balls
        for i in &keys {
            let ball = self.balls.get_mut(&i).unwrap();

            //gravity
            ball.force += Vector2::new(0.0f32, GRAVITY);
        }

        //update balls
        for i in &keys {
            self.balls.get_mut(&i).unwrap().update(dt);
        }
    }
    //assuming that a and b are in contact
    fn do_collision(a: &mut ball::Ball, b: &mut ball::Ball) {
        //check to make sure infinite collision loops dont happen by making sure balls are headed towards each other
        let diff = a.pos - b.pos;
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

    pub fn add_ball(&mut self, ball: ball::Ball) {
        self.balls.insert(self.current_index, ball);
        self.current_index += 1;
        //TODO check for collisions
    }

    pub fn get_balls(&self) -> &HashMap<i64, ball::Ball> {
        &self.balls
    }

    pub fn get_sectors(&self) -> &HashMap<(i32, i32), Vec<i64>> {
        &self.sectors
    }
}