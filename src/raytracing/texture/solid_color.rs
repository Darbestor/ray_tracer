use crate::math::vec3::Vec3;

use super::Texture;

pub struct SolidColor {
    pub color: Vec3,
}

impl SolidColor {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color: Vec3::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f32, _: f32, _: &Vec3) -> Vec3 {
        self.color
    }
}
