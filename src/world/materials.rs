extern crate rand;
use rand::Rng;

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

pub struct Dielectric {
    refraction_index: f32
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric {
            refraction_index
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected = reflect(ray_in.direction(), rec.normal);
        let attenuation = Vec3::one();
        let outward_normal;
        let ni_over_nt;
        let d = dot(ray_in.direction(), rec.normal);
        let cosine = {
            if d > 0.0 {
                outward_normal = -rec.normal;
                ni_over_nt = self.refraction_index;
                self.refraction_index * d / ray_in.direction().length()
            }
            else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / self.refraction_index;
                -d / ray_in.direction().length()
            }
        };
        let mut refracted = Vec3::zero();
        let reflection_probability = {
            if let Some(r) = refract(ray_in.direction(), outward_normal, ni_over_nt) {
                refracted = r;
                shclick(cosine, self.refraction_index)
            }
            else {
                1.0
            }
        };
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < reflection_probability {
            let scattered = Ray::new(rec.p, reflected);
            (true, attenuation, scattered)
        }
        else {
            let scattered = Ray::new(rec.p, refracted);
            (true, attenuation, scattered)
        }
    }
}
