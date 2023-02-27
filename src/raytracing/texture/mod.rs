pub mod solid_color;

use crate::math::vec3::Vec3;

pub trait Texture {
    /// Get texture value by given coordinates
    ///
    /// `u` - texture 'x' coord
    /// `v` - texture 'y' coord
    /// `point` - point on shape
    fn value(&self, u: f32, v: f32, point: &Vec3) -> Vec3;
}

pub struct UvCoords {
    pub u: f32,
    pub v: f32,
}

pub trait UvMapper {
    fn get_uv_coords(&self, normal: &Vec3) -> UvCoords;
}
