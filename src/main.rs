extern crate rand;
use rand::Rng;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::f32;

use std::rc::Rc;

mod math;
use math::vec3::Vec3;
use math::ray::Ray;

mod world;
use world::hitable::*;
use world::camera::*;
use world::materials::*;

fn color(r: Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(&r, 0.001, f32::MAX, &mut rec) {
        if depth < 50 {
            match rec.material {
                None => {
                    Vec3::zero()
                },
                Some (ref material) => {
                    let (scatter_result, attenuation, scattered) = material.scatter(&r, &rec);
                    if scatter_result {
                        attenuation * color(scattered, world, depth + 1)
                    }
                    else {
                        Vec3::zero()
                    }
                }
            }
        }
        else {
            Vec3::zero()
        }
    }
    else {
        let unit_direction = math::vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let file_name = "outpu.ppm";
    let file_path = Path::new(file_name);
    let mut file = match File::create(file_path) {
        Ok(file)    => file,
        Err(e)      => panic!("Could not create file: {} error: {:?}", file_name, e.kind()),
    };

    let width = 800;
    let height = 400;

    write!(file, "P3\n{} {}\n255\n", width,height).expect("Could not write to file");
    let hitable: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2))))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(1.5)))),
    ];

    let world = HitableList::new(hitable);
    let camera = Camera::new();

    let mut rng = rand::thread_rng();
    let num_samples = 100;
    
    for j in (0 .. height).rev() {
        for i in 0 .. width {
            let mut col = Vec3::zero();
            for _s in 0..num_samples {
                let u = (i as f32 + rng.gen::<f32>()) / (width as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (height as f32);
                let ray = camera.get_ray(u,v);
                col += color(ray, &world, 0);
            }
            col /= num_samples as f32;
            let r = (255.99 * col[0].sqrt()) as i32;
            let g = (255.99 * col[1].sqrt()) as i32;
            let b = (255.99 * col[2].sqrt()) as i32;
            write!(file, "{} {} {}\n", r, g, b).expect("Could not write to file");;
        }
    }
}
