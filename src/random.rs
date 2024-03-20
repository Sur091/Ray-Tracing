use glam::Vec3;
use rand::{distributions::uniform::SampleUniform, Rng};

pub fn number<T: SampleUniform + PartialOrd>(min: T, max: T) -> T {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn vec3(min: f32, max: f32) -> Vec3 {
    Vec3::new(number(min, max), number(min, max), number(min, max))
}

pub fn vec3_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(number(-1.0, 1.0), number(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn vec3_in_unit_sphere() -> Vec3 {
    loop {
        let p = vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn unit_vector() -> Vec3 {
    vec3_in_unit_sphere().normalize()
}
