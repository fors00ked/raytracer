use crate::math::vec3::Vec3;
use crate::math::ray::Ray;
use crate::math::vec3::dot as dot;
use super::materials::Material;

use std::vec::Vec;
use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
            material: None
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center, radius, material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let d = b * b - a * c;
        if d > 0.0 {
            let temp = (-b - d.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(Rc::clone(&self.material));
                return true
            }
            let temp = (-b + d.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(Rc::clone(&self.material));
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
                rec.material = temp_rec.material.as_ref().and_then(|x| Some(Rc::clone(x)));
            }
        }
        hit
    }
}

