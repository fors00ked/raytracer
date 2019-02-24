extern crate rand;
    use rand::Rng;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::f32;

mod math;
use math::vec3::Vec3;
use math::ray::Ray;

mod world;
use world::hitable::*;
use world::camera::*;

fn color(r: Ray, world: &dyn Hitable) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(&r, 0.001, f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + math::vec3::random_in_unit_sphere();
        return 0.5 * color(Ray::new(rec.p, target - rec.p), world)
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

    let nx = 800;
    let ny = 400;
    let ns = 100;

    write!(file, "P3\n{} {}\n255\n", nx, ny).expect("Could not write to file");
    let hitable: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    ];

    let world = HitableList::new(hitable);
    let camera = Camera::new();

    let mut rng = rand::thread_rng();
    
    for j in (0 .. ny).rev() {
        for i in 0 .. nx {
            let mut col = Vec3::zero();
            for _s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / (nx as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (ny as f32);
                let ray = camera.get_ray(u,v);
                col += color(ray, &world);
            }
            col /= ns as f32;
            let r = (255.99 * col[0].sqrt()) as i32;
            let g = (255.99 * col[1].sqrt()) as i32;
            let b = (255.99 * col[2].sqrt()) as i32;
            write!(file, "{} {} {}\n", r, g, b).expect("Could not write to file");;
        }
    }
}
