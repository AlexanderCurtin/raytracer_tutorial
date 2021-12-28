use std::{
    fmt::Write,
    io::{stdout, BufWriter, Write as IoWrite},
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

type Point3 = Vec3;
type Color = Vec3;

impl Vec3 {
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)
    }

    fn write_color<W: std::io::Write>(&self, writer: &mut BufWriter<W>) -> () {
        writeln!(
            writer,
            "{} {} {}",
            (self.x * 255.999) as i32,
            (self.y * 255.999) as i32,
            (self.z * 255.999) as i32
        )
        .unwrap();
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

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unimplemented!("Out of bounds"),
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        };
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = rhs + *self;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = rhs + *self;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        return self.origin + t * self.direction;
    }
    fn color(&self) -> Color {
        const SPHERE_CENTER: Point3 = Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        };
        let t = hit_sphere(&SPHERE_CENTER, 0.5, &self);
        if t > 0.0 {
            let N = unit_vector(
                &(self.at(t)
                    - Vec3 {
                        x: 0.,
                        y: 0.,
                        z: -1.,
                    }),
            );
            return 0.5
                * Color {
                    x: N.x + 1.,
                    y: N.y + 1.,
                    z: N.z + 1.,
                };
        }
        let unit_direction = unit_vector(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.);
        let _left = (1. - t)
            * Color {
                x: 1.,
                y: 1.,
                z: 1.,
            };
        let _right: Vec3 = t * Color {
            x: 0.5,
            y: 0.7,
            z: 1.,
        };
        let _ans = _left + _right;
        let real_ans = ((1. - t)
            * Color {
                x: 1.,
                y: 1.,
                z: 1.,
            })
            + t * Color {
                x: 0.5,
                y: 0.7,
                z: 1.,
            };
        return real_ans;
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0. {
        return -1.0;
    };
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera

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
            let pixel_color = r.color();
            pixel_color.write_color(&mut writer);
        }
    }
}
