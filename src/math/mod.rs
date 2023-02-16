use std::f32::consts::PI;

use self::vec3::Vec3;

/// Math primitives and oparations with them
pub mod vec3;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::random(-1., 1.);
        if v.length_squared() < 1. {
            break v;
        }
    }
}
