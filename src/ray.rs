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
    pub fn color(&self, world: &dyn Hit, depth: usize) -> Color {
        if depth <= 0 {
            return Color::from(0.);
        }
        let hit = world.hit(self, 0.001, INFINITY);
        if hit.is_none() {
            let unit_direction = unit_vector(&self.direction);
            let t = 0.5 * (unit_direction.y + 1.);
            return (1. - t) * Color::from(1.) + t * Color::new(0.5, 0.7, 1.0);
        }

        let rec = hit.unwrap();

        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(self, &rec) {
            return attenuation * scattered.color(world, depth - 1);
        }
        return Color::from(0.);
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}
