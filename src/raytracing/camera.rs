use crate::math::{degrees_to_radians, random_in_unit_disk, vec3::Vec3};

use super::ray::Ray;

pub struct Camera {
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub origin: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookup: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookup).norm();
        let u = vup.cross(&w).norm();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let origin = lookfrom;

        let lower_left_corner = origin - &horizontal / 2. - &vertical / 2. - focus_dist * w;

        Self {
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius: aperture / 2.,
            w,
            u,
            v,
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

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x() * self.u + rd.y() * self.v;

        let direction = self.lower_left_corner() + &(x * self.horizontal()) + y * self.vertical()
            - self.origin
            - offset;
        Ray::new(self.origin + offset, direction)
    }
}
