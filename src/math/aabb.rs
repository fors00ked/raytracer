use super::vec3::Vec3;
use super::ray::Ray;

pub fn surrounding_box(box1: Aabb, box2: Aabb) -> Aabb {
    let min = Vec3::new(
        box1.min().x().min(box2.min().x()),
        box1.min().y().min(box2.min().y()),
        box1.min().z().min(box2.min().z()));

    let max = Vec3::new(
        box1.max().x().max(box2.max().x()),
        box1.max().y().max(box2.max().y()),
        box1.max().z().max(box2.max().z()));

    Aabb::new(min, max)
}

#[derive (Copy, Clone)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb {
            min,
            max,
        }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut min = t_min;
        let mut max = t_max;
        let origin = r.origin();
        for i in 0..3 {
            let t0 = ((self.min[i] - origin[i]) / r.direction()[i]).min((self.max[i] - origin[i]) / r.direction()[i]);
            let t1 = ((self.min[i] - origin[i]) / r.direction()[i]).max((self.max[i] - origin[i]) / r.direction()[i]);
            min = t0.max(min);
            max = t1.min(max);
            if max <= min {
                return false
            }
        }
        true
    }
}
