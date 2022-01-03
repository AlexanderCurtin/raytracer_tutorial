pub trait Material {
    pub fn scatter(&self, r_in: &Ray, attenuation: &Color) -> Option<Ray>;
}
