mod camera;
mod constants;
mod vec3;
use std::{
    io::{stdout, BufWriter},
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
};

use constants::{INFINITY, PI};
use vec3::{Color, Point3, Vec3};

use crate::camera::Camera;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hit for Vec<Rc<dyn Hit>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None as Option<HitRecord>;
        let mut closest_so_far = t_max;
        for o in self.iter() {
            if let Some(temp_rec) = o.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        return rec;
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min || t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min || t_max < root) {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            p: r.at(root),
            normal: (p - self.center) / self.radius,
            t: root,
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.z,
    }
}

fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return u.x * v.x + u.y * v.y + u.z * v.z;
}

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        return self.origin + t * self.direction;
    }
    fn color(&self, world: &dyn Hit) -> Color {
        if let Some(rec) = world.hit(self, 0., INFINITY) {
            return 0.5
                * (rec.normal
                    + Color {
                        x: 1.,
                        y: 1.,
                        z: 1.,
                    });
        }
        let unit_direction = unit_vector(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.);
        return (1. - t)
            * Color {
                x: 1.,
                y: 1.,
                z: 1.,
            }
            + t * Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // World
    let mut world: Vec<Rc<dyn Hit>> = Vec::new();

    world.push(Rc::new(Sphere {
        center: Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        radius: 0.5,
    }));

    world.push(Rc::new(Sphere {
        center: Point3 {
            x: 0.,
            y: -100.5,
            z: -1.,
        },
        radius: 100.,
    }));

    // Camera

    let camera = Camera::new();

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.,
        z: 0.,
    };

    let vertical = Vec3 {
        x: 0.,
        y: viewport_height,
        z: 0.,
    };

    let lower_left_corner = origin
        - horizontal / 2f64
        - vertical / 2f64
        - Vec3 {
            x: 0.,
            y: 0.,
            z: focal_length,
        };

    print!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut writer = BufWriter::new(stdout());
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = r.color(&world);
            pixel_color.write_color(&mut writer);
        }
    }
}
