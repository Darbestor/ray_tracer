use crate::math::vec3::Vec3;

pub struct Camera {
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    pub origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(viewport_height: f32, viewport_width: f32) -> Self {
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let origin = Vec3::zero();
        let focal_length = 1.0;

        let lower_left_corner =
            origin - &horizontal / 2. - &vertical / 2. - Vec3::new(0., 0., focal_length);

        Self {
            viewport_height,
            viewport_width,
            focal_length: 1.0,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub const fn horizontal(&self) -> &Vec3 {
        &self.horizontal
    }

    pub const fn vertical(&self) -> &Vec3 {
        &self.vertical
    }

    pub const fn lower_left_corner(&self) -> &Vec3 {
        &self.lower_left_corner
    }
}
