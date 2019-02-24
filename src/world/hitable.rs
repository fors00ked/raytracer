use crate::math::vec3::Vec3;
use crate::math::ray::Ray;
use crate::math::vec3::dot as dot;

use std::vec::Vec;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere {
            center, radius
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = 2.0 * dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        if d > 0.0 {
            let temp = (-b - d.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true
            }
            let temp = (-b + d.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true
            }
        }
        false
    }
}

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> Self {
        HitableList {
            list
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit = false;
        let mut closest = t_max;
        for l in self.list.iter() {
            if l.hit(r, t_min, closest, &mut temp_rec) {
                hit = true;
                closest = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        hit
    }
}

