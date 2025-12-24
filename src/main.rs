mod ray;
mod hittable;
mod sphere;
mod material;
mod camera;

use std::sync::Arc;
use glam::Vec3;
use image::{Rgb, RgbImage};
use rand::Rng;
use rayon::prelude::*;

use ray::Ray;
use hittable::{HitRecord, Hittable};
use sphere::Sphere;
use material::{Lambertian, Metal, DiffuseLight};
use camera::Camera;


struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self { Self { objects: Vec::new() } }
    fn add(&mut self, object: Box<dyn Hittable>) { self.objects.push(object); }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }
        hit_anything
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 { return Vec3::ZERO; }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {

        let emitted = rec.mat.emitted();

        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {

            return emitted + attenuation * ray_color(&scattered, world, depth - 1);
        } else {

            return emitted;
        }
    }

    Vec3::ZERO 
}

fn main() {
    // 画像設定
    let aspect_ratio = 16.0 / 9.0;
    let width = 800;
    let height = (width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

    let mut img = RgbImage::new(width, height);

    // シーン
    let material_ground = Arc::new(Lambertian { albedo: Vec3::new(0.8, 0.8, 0.8) });
    let material_center = Arc::new(Lambertian { albedo: Vec3::new(0.1, 0.2, 0.5) });
    let material_left   = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right  = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let material_light  = Arc::new(DiffuseLight { emit: Vec3::new(10.0, 10.0, 10.0) });

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-2.0, 2.0, 0.0), 2.0, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(2.0, 2.0, 0.0), 2.0, material_right)));

    //光源
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, material_light)));

    let lookfrom = Vec3::new(26.0, 20.0, 10.0); // 遠くから見る
    let lookat = Vec3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom, 
        lookat, 
        vup, 
        20.0, 
        aspect_ratio,
        aperture,
        dist_to_focus
    );

    println!("レンダリング開始...");
    img.enumerate_pixels_mut()
        .par_bridge() 
        .for_each(|(x, y, pixel)| {
            
            let mut rng = rand::thread_rng();

            let mut pixel_color = Vec3::ZERO;
            for _ in 0..samples_per_pixel {

                let u_offset: f32 = rng.r#gen();
                let v_offset: f32 = rng.r#gen();
                
                let u = (x as f32 + u_offset) / (width as f32 - 1.0);
                let v = ((height as f32 - 1.0) - y as f32 + v_offset) / (height as f32 - 1.0);
                
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            let scale = 1.0 / samples_per_pixel as f32;
            let r = (pixel_color.x * scale).sqrt();
            let g = (pixel_color.y * scale).sqrt();
            let b = (pixel_color.z * scale).sqrt();

            *pixel = Rgb([
                (r * 256.0 - 0.001).max(0.0) as u8,
                (g * 256.0 - 0.001).max(0.0) as u8,
                (b * 256.0 - 0.001).max(0.0) as u8,
            ]);
        });

    img.save("output.png").unwrap();
    println!("saveed output.png");
}
