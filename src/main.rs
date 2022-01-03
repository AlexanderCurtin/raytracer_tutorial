mod camera;
mod constants;
mod hit;
mod material;
mod ray;
mod utils;
mod vec3;
use std::{
    io::{stdout, BufWriter},
    rc::Rc,
};

use hit::Hit;

use vec3::{Point3, Vec3};

use crate::{
    camera::Camera,
    hit::Sphere,
    material::{Lambertian, Metal},
    ray::Ray,
    utils::random_double,
    vec3::Color,
};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let material_groud = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::from(0.8),
    });

    let material_right = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    });

    // World
    let mut world: Vec<Rc<dyn Hit>> = Vec::new();

    world.push(Rc::new(Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
        mat_ptr: material_groud,
    }));

    world.push(Rc::new(Sphere {
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
        mat_ptr: material_center,
    }));

    world.push(Rc::new(Sphere {
        center: Point3::new(-1., 0., -1.),
        radius: 0.5,
        mat_ptr: material_left,
    }));

    world.push(Rc::new(Sphere {
        center: Point3::new(1., 0., -1.),
        radius: 0.5,
        mat_ptr: material_right,
    }));

    // Camera

    let camera = Camera::new();

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
                pixel_color += r.color(&world, MAX_DEPTH);
            }
            pixel_color.write_color(&mut writer, SAMPLES_PER_PIXEL);
        }
    }
}
