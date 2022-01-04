use crate::{
    hit::HitRecord,
    ray::Ray,
    utils::{dot, reflect, unit_vector},
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Color,
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        return Some((self.albedo, scattered));
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&unit_vector(&r_in.direction), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + (self.fuzz * Vec3::random_in_unit_sphere()),
        );
        let attenuation = self.albedo;
        if dot(&scattered.direction, &rec.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
