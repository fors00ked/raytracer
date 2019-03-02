use std::f32;

use crate::math::ray::Ray;
use crate::math::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, vfov_degrees: f32, aspect: f32) -> Camera {
        let theta = vfov_degrees.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(up, w));
        let v = cross(w, u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}