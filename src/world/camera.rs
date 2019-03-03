use std::f32;

use crate::math::ray::Ray;
use crate::math::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, vfov_degrees: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov_degrees.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(up, w));
        let v = cross(w, u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * focus_dist * u - half_height * focus_dist * v - w * focus_dist,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
            _w: w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}