use crate::math::ray::Ray;
use crate::math::vec3::*;
use super::hitable::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray);
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        (true, self.albedo, scattered)
    }
}

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected = reflect(unit_vector(ray_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        (dot(scattered.direction(), rec.normal) > 0.0, self.albedo, scattered)
    }
}