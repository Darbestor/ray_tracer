pub mod checker;
pub mod image;
pub mod solid_color;

use crate::math::vec3::Vec3;

pub trait Texture {
    /// Get texture value by given coordinates
    ///
    /// `UvCoords` - texture coordinates on surfacee
    /// `point` - point on shape
    fn value(&self, uv_coords: &UvCoords, point: &Vec3) -> Vec3;
}

/// Texture coordinates
pub struct UvCoords {
    // `x` axis coord
    pub u: f32,
    // `y` axis coord
    pub v: f32,
}

pub trait UvMapper {
    fn get_uv_coords(&self, normal: &Vec3) -> UvCoords;
}
