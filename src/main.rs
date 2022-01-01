mod camera;
mod constants;
mod hit;
mod ray;
mod utils;
mod vec3;
use std::{
    io::{stdout, BufWriter},
    rc::Rc,
};

use hit::Hit;

use vec3::{Point3, Vec3};

use crate::{camera::Camera, hit::Sphere, ray::Ray, utils::random_double, vec3::Color};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;

    // World
    let mut world: Vec<Rc<dyn Hit>> = Vec::new();

    world.push(Rc::new(Sphere {
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
    }));

    world.push(Rc::new(Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
    }));

    // Camera

    let camera = Camera::new();

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin: Vec3 = (0.).into();
    let horizontal = Vec3::new(viewport_width, 0., 0.);

    let vertical = Vec3 {
        x: 0.,
        y: viewport_height,
        z: 0.,
    };

    print!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut writer = BufWriter::new(stdout());
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::from(0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += r.color(&world);
            }
            pixel_color.write_color(&mut writer, SAMPLES_PER_PIXEL);
        }
    }
}
