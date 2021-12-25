use nalgebra::Vector2;

pub enum Mat {
    Custom,
    Polar,
    Anion,
    Cation,
}

pub struct Ball {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub rad: f32,
    pub range: f32, //interaction radius for ball
    pub mass: f32,
    pub force: Vector2<f32>,
    pub mat: Mat,
}

impl Ball {
    //constructor
    pub fn new(x_pos: f32, y_pos: f32, x_vel: f32, y_vel: f32, rad: f32, mass: f32) -> Ball {
        let pos = Vector2::new(x_pos, y_pos);
        let vel = Vector2::new(x_vel, y_vel);
        let force = Vector2::new(0.0f32, 0.0f32);
        Ball { pos, vel, rad, range: rad, mass, force, mat: Mat::Custom }
    }

    pub fn blank() -> Ball {
        Ball::new(0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32)
    }
    //physics update function
    pub fn update(&mut self, dt: f32) {
        self.vel += self.force * dt;
        self.pos += self.vel * dt;
        self.force = Vector2::new(0.0f32, 0.0f32);
    }
    //builder functions
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.pos = Vector2::new(x, y);
        self
    }

    pub fn with_vel(mut self, x: f32, y: f32) -> Self {
        self.vel = Vector2::new(x, y);
        self
    }

    pub fn with_rad(mut self, rad: f32) -> Self {
        self.rad = rad;
        self
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    fn with_mat(mut self, mat: Mat) -> Self {
        self.mat = mat;
        self
    }

    fn with_range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }
    //initializers
    pub fn polar() -> Ball {
        Ball::blank().
            with_mat(Mat::Polar).
            with_range(2.0)
    }

    pub fn cation() -> Ball {
        Ball::blank().
            with_mat(Mat::Cation).
            with_range(2.0)
    }

    pub fn anion() -> Ball {
        Ball::blank().
            with_mat(Mat::Anion).
            with_range(2.0)
    }
}