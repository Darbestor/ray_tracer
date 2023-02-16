use std::f32::consts::PI;

/// Math primitives and oparations with them
pub mod vec3;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
