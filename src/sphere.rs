use std::sync::Arc;
use glam::Vec3;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Arc<dyn Material + Send + Sync>, 
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 { return None; }
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root { return None; }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        
        let mut rec = HitRecord {
            t: root,
            p,
            normal: Vec3::ZERO,
            front_face: false,
            mat: self.mat.clone(),
        };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}