use nalgebra::Vector2;

use super::ball::Ball;
use super::ball::Mat;

//force that b exerts on a through field forces
pub fn get_force(a: &Ball, b: &Ball) -> Vector2<f32>{
    let pair = (&a.mat, &b.mat);
    match pair {
        (Mat::Polar, Mat::Polar) => polar_polar(&a, &b),
        (Mat::Cation, Mat::Cation) => cation_cation(&a, &b),
        (Mat::Anion, Mat::Anion) => anion_anion(&a, &b),
        (Mat::Anion, Mat::Cation) => anion_cation(&a, &b),
        (Mat::Cation, Mat::Anion) => -anion_cation(&b, &a),
        _ => Vector2::new(0.,0.)
    }
}

fn polar_polar(a: &Ball, b: &Ball) -> Vector2<f32> {
    let diff: Vector2<f32> = b.pos - a.pos;
    if diff.norm_squared() > 16. {
        return Vector2::new(0., 0.)
    }
    let ns = diff.norm_squared();
    let att = diff / (ns + 1.);
    let correct = -diff / (17.);
    (att + correct) * 2.
}

fn anion_cation(a: &Ball, b: &Ball) -> Vector2<f32> {
    let diff: Vector2<f32> = b.pos - a.pos;
    if diff.norm_squared() > 16. {
        return Vector2::new(0., 0.)
    }
    let ns = diff.norm_squared();
    let att = diff / (ns + 1.);
    let correct = -diff / (17.);
    (att + correct) * 16.
}

fn cation_cation(a: &Ball, b: &Ball) -> Vector2<f32> {
    let diff: Vector2<f32> = b.pos - a.pos;
    if diff.norm_squared() > 16. {
        return Vector2::new(0., 0.)
    }
    let ns = diff.norm_squared();
    let att = -diff / (ns + 1.);
    let correct = diff / (17.);
    (att + correct) * 16.
}

fn anion_anion(a: &Ball, b: &Ball) -> Vector2<f32> {
    let diff: Vector2<f32> = b.pos - a.pos;
    if diff.norm_squared() > 16. {
        return Vector2::new(0., 0.)
    }
    let ns = diff.norm_squared();
    let att = -diff / (ns + 1.);
    let correct = diff / (17.);
    (att + correct) * 16.
}