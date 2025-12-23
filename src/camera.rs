use glam::Vec3;
use rand::Rng; // 追加
use crate::ray::Ray;

// 単位円盤内のランダムな点を返す（レンズ上の発射位置を決めるため）
fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.r#gen::<f32>() * 2.0 - 1.0, rng.r#gen::<f32>() * 2.0 - 1.0, 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, // カメラの横方向ベクトルを保存しておく（オフセット計算用）
    v: Vec3, // カメラの縦方向ベクトル
    lens_radius: f32, // レンズの半径
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,    // 追加: 絞り（レンズの直径のようなもの）
        focus_dist: f32,  // 追加: ピントが合う距離
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        
        // ★重要: viewportの大きさを focus_dist 倍する
        // これにより、focus_dist の距離にある物体だけが元の大きさで（ピントが合って）映る
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        // レンズ上のランダムな位置を計算
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset, // 発射地点をずらす
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}