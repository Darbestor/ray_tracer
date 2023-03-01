pub mod checker;
pub mod image;
pub mod solid_color;

use crate::math::vec3::Vec3;

pub use self::{checker::CheckerTexture, image::ImageTexture, solid_color::SolidColorTexture};

pub enum Texture {
    SolidColor(SolidColorTexture),
    Checker(CheckerTexture),
    Image(ImageTexture),
}

/// Common functionality for textures
pub trait TextureFunc {
    /// Get texture value by given coordinates
    ///
    /// `UvCoords` - texture coordinates on surfacee
    /// `point` - point on shape
    fn value(&self, uv_coords: &UvCoords, point: &Vec3) -> Vec3;
}

pub trait UvMapper {
    fn get_uv_coords(&self, normal: &Vec3) -> UvCoords;
}

/// Texture coordinates
pub struct UvCoords {
    // `x` axis coord
    pub u: f32,
    // `y` axis coord
    pub v: f32,
}

impl TextureFunc for Texture {
    fn value(&self, uv_coords: &UvCoords, point: &Vec3) -> Vec3 {
        match self {
            Texture::SolidColor(tex) => tex.value(uv_coords, point),
            Texture::Checker(tex) => tex.value(uv_coords, point),
            Texture::Image(tex) => tex.value(uv_coords, point),
        }
    }
}
