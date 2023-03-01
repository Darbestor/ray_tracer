use std::sync::Arc;

use crate::{
    math::vec3::Vec3,
    raytracing::{
        aabb::BoundingBox,
        material::Material,
        ray_hit::{HitResult, Normal, RayHitTester},
        texture::{UvCoords, UvMapper},
    },
};

use super::HittableObject;

pub struct Plane {
    pub x_start: f32,
    pub y_start: f32,
    pub z: f32,
    pub x_end: f32,
    pub y_end: f32,
    pub material: Arc<Material>,
}

impl Plane {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, material: Arc<Material>) -> Self {
        Self {
            x_start: x,
            y_start: y,
            z,
            x_end: x + width,
            y_end: y + height,
            material,
        }
    }

    fn get_uv(&self, x: &f32, y: &f32) -> UvCoords {
        let u = (x - self.x_start) / (self.x_end - self.x_start);
        let v = (y - self.y_start) / (self.y_end - self.y_start);
        UvCoords { u, v }
    }
}

impl RayHitTester for Plane {
    fn hit(
        &self,
        ray: &crate::raytracing::ray::Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<HitResult> {
        let depth = (self.z - ray.origin.z()) / ray.direction.z();
        if depth < min_distance || depth > max_distance {
            return None;
        }

        // auto x = r.origin().x() + t*r.direction().x();
        // auto y = r.origin().y() + t*r.direction().y();
        // if (x < x0 || x > x1 || y < y0 || y > y1)
        //     return false;
        // rec.u = (x-x0)/(x1-x0);
        // rec.v = (y-y0)/(y1-y0);
        // rec.t = t;
        // auto outward_normal = vec3(0, 0, 1);
        // rec.set_face_normal(r, outward_normal);
        // rec.mat_ptr = mp;
        // rec.p = r.at(t);
        // return true;

        let x = ray.origin.x() + depth * ray.direction.x();
        let y = ray.origin.y() + depth * ray.direction.y();
        if x < self.x_start || x > self.x_end || y < self.y_start || y > self.y_end {
            return None;
        }

        let location = ray.at(depth);
        let mut normal = Vec3::new(0., 0., 1.);
        let front_face = ray.direction.dot(&normal) < 0.;
        if !front_face {
            normal = -normal;
        }
        Some(HitResult {
            location,
            normal,
            distance: depth,
            front_face,
            material: self.material.clone(),
            uv: self.get_uv(&x, &y),
        })
    }
}

impl HittableObject for Plane {}

impl BoundingBox for Plane {
    fn bounding_box(
        &self,
        _: f32,
        _: f32,
    ) -> Result<crate::raytracing::aabb::AABB, crate::raytracing::aabb::BoundingBoxError> {
        todo!()
    }
}
