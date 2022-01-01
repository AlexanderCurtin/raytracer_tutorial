use crate::{
    constants::INFINITY,
    hit::Hit,
    utils::unit_vector,
    vec3::{Color, Point3, Vec3},
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin + t * self.direction;
    }
    pub fn color(&self, world: &dyn Hit) -> Color {
        if let Some(rec) = world.hit(self, 0., INFINITY) {
            return 0.5 * (rec.normal + Color::from(1.));
        }
        let unit_direction = unit_vector(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.);
        return (1. - t) * Color::from(1.) + t * Color::new(0.5, 0.7, 1.0);
    }
}
