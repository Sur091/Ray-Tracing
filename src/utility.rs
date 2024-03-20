use glam::Vec3;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (v.dot(n) * 2.0)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = 1.0f32.min(-uv.dot(n));
    let r_out_perpendicular = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perpendicular.length_squared()).abs().sqrt());
    r_out_parallel + r_out_perpendicular
}
