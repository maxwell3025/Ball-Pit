use nalgebra::Vector2;

use super::ball::Ball;

//force that b exerts on a through field forces
pub fn get_force(a: &Ball, b: &Ball) -> Vector2<f32>{
    let pair = (&a.mat, &b.mat);
    match pair {
        _ => Vector2::new(0.,0.)
    }
}