use crate::math::vec3::Vec3;

use super::{TextureFunc, UvCoords};

pub struct SolidColorTexture {
    pub color: Vec3,
}

impl SolidColorTexture {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color: Vec3::new(red, green, blue),
        }
    }
}

impl TextureFunc for SolidColorTexture {
    fn value(&self, _: &UvCoords, _: &Vec3) -> Vec3 {
        self.color
    }
}

impl From<Vec3> for SolidColorTexture {
    fn from(value: Vec3) -> Self {
        Self { color: value }
    }
}
