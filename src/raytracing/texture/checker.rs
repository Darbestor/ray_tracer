use std::sync::Arc;

use super::{solid_color::SolidColorTexture, TextureFunc};

pub struct CheckerTexture {
    pub odd: Arc<SolidColorTexture>,
    pub even: Arc<SolidColorTexture>,
}

impl CheckerTexture {
    pub fn new(odd_color: Arc<SolidColorTexture>, even_color: Arc<SolidColorTexture>) -> Self {
        Self {
            odd: odd_color,
            even: even_color,
        }
    }
}

impl TextureFunc for CheckerTexture {
    fn value(
        &self,
        uv_coords: &super::UvCoords,
        point: &crate::math::vec3::Vec3,
    ) -> crate::math::vec3::Vec3 {
        // auto sines = sin(10*p.x())*sin(10*p.y())*sin(10*p.z());
        // if (sines < 0)
        //     return odd->value(u, v, p);
        // else
        //     return even->value(u, v, p);
        let sines =
            f32::sin(10.0 * point.x()) * f32::sin(10.0 * point.y()) * f32::sin(10.0 * point.z());
        if sines < 0.0 {
            self.odd.value(uv_coords, point)
        } else {
            self.even.value(uv_coords, point)
        }
    }
}
