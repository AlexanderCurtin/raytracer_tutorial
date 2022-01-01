use rand::{distributions::Uniform, prelude::Distribution};

use crate::{constants::PI, vec3::Vec3};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    Uniform::<f64>::new(0.0, 1.).sample(&mut rng)
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.z,
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return u.x * v.x + u.y * v.y + u.z * v.z;
}
