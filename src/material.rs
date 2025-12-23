use glam::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

// 散乱（Scatter）の結果を返すためのヘルパー関数
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

// 単位球内のランダムなベクトル（main.rsから移動してもOKですが、ここでは再定義します）
fn random_in_unit_sphere() -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        if p.length_squared() < 1.0 { return p; }
    }
}

pub trait Material {
    // レイが当たったときに、どう散乱するか（反射するか）を計算する
    // 戻り値: Option<(減衰率(色), 散乱後のレイ)>
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;

    fn emitted(&self) -> Vec3 {
        Vec3::ZERO
    }
}

// 1. 拡散反射（マットな質感）
pub struct Lambertian {
    pub albedo: Vec3, // 反射率（色）
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere().normalize();

        // 稀にランダムベクトルが法線と真逆でキャンセルされゼロになるのを防ぐ
        if scatter_direction.length_squared() < 1e-8 {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

// 2. 金属（鏡面反射）
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32, // 表面の粗さ（0.0=鏡面, 1.0=かなり曇る）
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Self {
        Self { albedo, fuzz: if f < 1.0 { f } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        // 粗さ(fuzz)の分だけ反射方向をずらす
        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        
        // 散乱方向が表面の内側に向いてしまった場合は吸収（反射しない）とする
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
    // 光源自体はレイを散乱させない（そこで光の追跡が終わる＝光源が見える）
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None 
    }

    // 発光色を返す
    fn emitted(&self) -> Vec3 {
        self.emit
    }
}