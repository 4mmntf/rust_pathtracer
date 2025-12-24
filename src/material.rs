use glam::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn random_in_unit_sphere() -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        if p.length_squared() < 1.0 { return p; }
    }
}

pub trait Material {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;

    fn emitted(&self) -> Vec3 {
        Vec3::ZERO
    }
}
pub struct Lambertian {
    pub albedo: Vec3, // 反射率
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere().normalize();

        if scatter_direction.length_squared() < 1e-8 {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Self {
        Self { albedo, fuzz: if f < 1.0 { f } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);

        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct DiffuseLight {
    pub emit: Vec3, // 発光色
}

impl Material for DiffuseLight {

    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None 
    }

    fn emitted(&self) -> Vec3 {
        self.emit
    }
}