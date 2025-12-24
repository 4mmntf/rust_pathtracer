// src/hittable.rs
use std::sync::Arc;
use glam::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material + Send + Sync>, 
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>, 
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    
    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
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