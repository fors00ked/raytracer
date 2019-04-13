extern crate rand;
use rand::Rng;

extern crate rayon;
use rayon::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::f32;
use std::sync::Arc;

mod math;
use math::vec3::Vec3;
use math::ray::Ray;

mod world;
use world::bvh::*;
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

fn random_scene() -> Vec<Arc<dyn Hitable+Send+Sync>> {
    let mut rng = rand::thread_rng();
    let mut hitable: Vec<Arc<dyn Hitable+Send+Sync>> = vec![];
    hitable.push(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            let rand = rng.gen::<f32>();
            if (center - Vec3::new(4.9, 0.2, 0.0)).length() > 0.9 {
                if rand < 0.8 {
                    hitable.push(Arc::new(Sphere::new(center, 0.2, Arc::new(Lambertian::new(Vec3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>()* rng.gen::<f32>()))))));
                }
                else if rand < 0.95 {
                    hitable.push(Arc::new(Sphere::new(center, 0.2, Arc::new(Metal::new(Vec3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())))))));
                }
                else {
                    hitable.push(Arc::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }
    hitable.push(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.3, 0.9, 0.4))))));
    hitable.push(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
    hitable.push(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5))))));
    hitable
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
    let mut hitable: Vec<Arc<Hitable+Send+Sync>> = random_scene();
        /*vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))))),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2))))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(1.5)))),
        ];*/

    let world = BvhNode::new(&mut hitable);
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 20.0, (width as f32) / (height as f32), 0.1, 10.0);

    let num_samples = 100;

    let mut color_buf = vec![vec![Vec3::zero(); width]; height];
    
    color_buf.par_iter_mut().enumerate().for_each(|(i, pixel)| {
        let mut rng = rand::thread_rng();
        for j in 0 .. width {
            let mut col = Vec3::zero();
            for _s in 0..num_samples {
                let u = (j as f32 + rng.gen::<f32>()) / (width as f32);
                let v = (i as f32 + rng.gen::<f32>()) / (height as f32);
                let ray = camera.get_ray(u,v);
                col += color(ray, &world, 0);
            }
            col /= num_samples as f32;
            (*pixel)[j] = col;
        }
    });

    for i in (0 .. height).rev() {
        for j in 0 .. width {
            let col = color_buf[i][j];
            let r = (255.99 * col[0].sqrt()) as i32;
            let g = (255.99 * col[1].sqrt()) as i32;
            let b = (255.99 * col[2].sqrt()) as i32;
            write!(file, "{} {} {}\n", r, g, b).expect("Could not write to file");
        }
    }
}
