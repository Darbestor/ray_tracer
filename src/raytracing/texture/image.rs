use std::{
    io::{self},
    path::Path,
};

use image::RgbImage;

use crate::math::vec3::Vec3;

use super::TextureFunc;

pub struct ImageTexture {
    image_buffer: RgbImage,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(filepath: P) -> Result<Self, io::Error> {
        let texture = image::open(filepath).unwrap().into_rgb8();
        Ok(Self {
            image_buffer: texture,
        })
    }
}

impl TextureFunc for ImageTexture {
    fn value(
        &self,
        uv_coords: &super::UvCoords,
        _: &crate::math::vec3::Vec3,
    ) -> crate::math::vec3::Vec3 {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = f32::clamp(uv_coords.u, 0.0, 1.0);
        // Flip V to image
        let v = 1.0 - f32::clamp(uv_coords.v, 0.0, 1.0);

        let mut x = (u * self.image_buffer.width() as f32) as u32;
        let mut y = (v * self.image_buffer.height() as f32) as u32;

        if x >= self.image_buffer.width() {
            x = self.image_buffer.width() - 1;
        }

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if y >= self.image_buffer.height() {
            y = self.image_buffer.height() - 1;
        }

        const COLOR_SCALE: f32 = 1.0 / 255.0;
        let pixel = self.image_buffer.get_pixel(x, y).0;
        // auto pixel = data + j*bytes_per_scanline + i*bytes_per_pixel;

        Vec3::new(
            pixel[0] as f32 * COLOR_SCALE,
            pixel[1] as f32 * COLOR_SCALE,
            pixel[2] as f32 * COLOR_SCALE,
        )
        // return color(color_scale*pixel[0], color_scale*pixel[1], color_scale*pixel[2]);
    }
}
