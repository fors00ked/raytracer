extern crate rand;
use rand::Rng;

use std::cmp::Ordering;
use std::sync::Arc;

use crate::math::aabb::*;
use crate::math::ray::*;
use super::hitable::*;

fn sort(list: &mut[Arc<dyn Hitable+Send+Sync>], axis: usize) {
    list.sort_by(
        |a, b| {
            let result1 = a.bounding_box();
            let result2 = b.bounding_box();
            if result1.result == false || result2.result == false {
                panic!()
            }
            if result1.aabb.min()[axis] - result2.aabb.min()[axis] < 0.0 {
                Ordering::Less
            }
            else {
                Ordering::Greater
            }
        }
    );
}

pub struct BvhNode {
    aabb: Aabb,
    left: Arc<dyn Hitable+Send+Sync>,
    right: Arc<dyn Hitable+Send+Sync>,
}

impl BvhNode {
    pub fn new(list: &mut[Arc<dyn Hitable+Send+Sync>]) -> BvhNode {
        let mut rng = rand::thread_rng();
        let axis: u32 = rng.gen_range(0, 3);
        if axis == 0 {
            sort(list, 0);
        }
        else if axis == 1 {
            sort(list, 1);
        }
        else {
            sort(list, 2);
        }

        let left: Arc<dyn Hitable+Send+Sync>;
        let right: Arc<dyn Hitable+Send+Sync>;
        if list.len() == 1 {
            left = Arc::clone(&list[0]);
            right = Arc::clone(&left);
        }
        else if list.len() == 1 {
            left = Arc::clone(&list[0]);
            right = Arc::clone(&list[1]);
        }
        else {
            let center = list.len() / 2;
            left = Arc::new(BvhNode::new(&mut list[0..center]));
            right = Arc::new(BvhNode::new(&mut list[center..]));
        }

        let left_result = left.bounding_box();
        let right_result = right.bounding_box();
        if left_result.result == false && right_result.result == false {
            panic!();
        }

        BvhNode {
            aabb: surrounding_box(left_result.aabb, right_result.aabb),
            left: Arc::clone(&left),
            right: Arc::clone(&right)
        }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32, rec: &mut HitRecord) -> bool {
        if self.aabb.hit(r, t_min, t_max) {
            let mut left_rec = HitRecord::new();
            let mut right_rec = HitRecord::new();
            let left_hit = self.left.hit(r, t_min, t_max, &mut left_rec);
            let right_hit = self.right.hit(r, t_min, t_max, &mut right_rec);
            if left_hit && right_hit {
                if left_rec.t < right_rec.t {
                    rec.t = left_rec.t;
                    rec.p = left_rec.p;
                    rec.normal = left_rec.normal;
                    rec.material = left_rec.material;
                    return true;
                }
                else {
                    rec.t = right_rec.t;
                    rec.p = right_rec.p;
                    rec.normal = right_rec.normal;
                    rec.material = right_rec.material;
                    return true;
                }
            }
            else if left_hit {
                rec.t = left_rec.t;
                rec.p = left_rec.p;
                rec.normal = left_rec.normal;
                rec.material = left_rec.material;
                return true;
            }
            else if right_hit {
                rec.t = right_rec.t;
                rec.p = right_rec.p;
                rec.normal = right_rec.normal;
                rec.material = right_rec.material;
                return true;
            }
        }
        false
    }

    fn bounding_box(&self) -> AabbResult {
        AabbResult {
            result: true,
            aabb: self.aabb
        }
    }
}