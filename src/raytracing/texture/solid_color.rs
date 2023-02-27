use crate::math::vec3::Vec3;

use super::{Texture, UvCoords};

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
    fn value(&self, _: &UvCoords, _: &Vec3) -> Vec3 {
        self.color
    }
}

impl From<Vec3> for SolidColor {
    fn from(value: Vec3) -> Self {
        Self { color: value }
    }
}
