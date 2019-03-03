extern crate rand;
use rand::Rng;

use std::f32;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;


pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        e: [
            v1.y() * v2.z() - v1.z() * v2.y(),
            -(v1.x() * v2.z() - v1.z() * v2.x()),
            v1.x() * v2.y() - v1.y() * v2.x()
        ]
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if dot(p, p) < 1.0 {
            return p
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = dot(uv, n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * d.sqrt();
        return Some(refracted)
    }
    None
}

pub fn shclick(cosine: f32, refraction_index: f32) -> f32{
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0_sq = r0 * r0;
    r0_sq + (1.0 - r0_sq) * (1.0 - cosine).powi(5)
}

#[derive(Debug, Copy,Clone)]
pub struct Vec3{
    e: [f32;3]
}

impl Vec3 {
    pub fn new(x:f32, y:f32, z: f32) -> Vec3 {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn one() -> Vec3 {
        Vec3 {
            e: [1.0, 1.0, 1.0]
        }
    }

    pub fn x(&self) -> f32{
        self.e[0]
    }

    pub fn y(&self) -> f32{
        self.e[1]
    }

    pub fn z(&self) -> f32{
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        Vec3 {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(& mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self {
        Vec3 {
            e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]]
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            e: [rhs * self.e[0], rhs * self.e[1], rhs * self.e[2]]
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs]
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1.0
    }
}
