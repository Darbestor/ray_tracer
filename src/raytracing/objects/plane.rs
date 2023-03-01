use std::sync::Arc;

use crate::{
    math::vec3::Vec3,
    raytracing::{
        aabb::BoundingBox,
        material::Material,
        ray_hit::{HitResult, RayHitTester},
        texture::UvCoords,
    },
};

use super::HittableObject;

pub struct PlaneZ {
    pub x_start: f32,
    pub x_end: f32,
    pub y_start: f32,
    pub y_end: f32,
    pub z: f32,
    pub material: Arc<Material>,
}

pub struct PlaneX {
    pub y_start: f32,
    pub y_end: f32,
    pub z_start: f32,
    pub z_end: f32,
    pub x: f32,
    pub material: Arc<Material>,
}

pub struct PlaneY {
    pub z_start: f32,
    pub z_end: f32,
    pub x_start: f32,
    pub x_end: f32,
    pub y: f32,
    pub material: Arc<Material>,
}

// ------ Plane Z -----------
impl PlaneZ {
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

impl RayHitTester for PlaneZ {
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

impl HittableObject for PlaneZ {}

impl BoundingBox for PlaneZ {
    fn bounding_box(
        &self,
        _: f32,
        _: f32,
    ) -> Result<crate::raytracing::aabb::AABB, crate::raytracing::aabb::BoundingBoxError> {
        todo!()
    }
}
// -----------------------

// --------- Plane X -----------------------
impl PlaneX {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, material: Arc<Material>) -> Self {
        Self {
            y_start: y,
            z_start: z,
            x,
            y_end: y + width,
            z_end: z + height,
            material,
        }
    }

    fn get_uv(&self, y: &f32, z: &f32) -> UvCoords {
        let u = (y - self.y_start) / (self.y_end - self.y_start);
        let v = (z - self.z_start) / (self.z_end - self.z_start);
        UvCoords { u, v }
    }
}

impl RayHitTester for PlaneX {
    fn hit(
        &self,
        ray: &crate::raytracing::ray::Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<HitResult> {
        let depth = (self.x - ray.origin.x()) / ray.direction.x();
        if depth < min_distance || depth > max_distance {
            return None;
        }

        let z = ray.origin.z() + depth * ray.direction.z();
        let y = ray.origin.y() + depth * ray.direction.y();
        if y < self.y_start || y > self.y_end || z < self.z_start || z > self.z_end {
            return None;
        }

        let location = ray.at(depth);
        let mut normal = Vec3::new(1., 0., 0.);
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
            uv: self.get_uv(&y, &z),
        })
    }
}

impl HittableObject for PlaneX {}

impl BoundingBox for PlaneX {
    fn bounding_box(
        &self,
        _: f32,
        _: f32,
    ) -> Result<crate::raytracing::aabb::AABB, crate::raytracing::aabb::BoundingBoxError> {
        todo!()
    }
}
// -------------------------------

// --------- Plane Y -----------------------
impl PlaneY {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, material: Arc<Material>) -> Self {
        Self {
            x_start: x,
            z_start: z,
            y,
            x_end: x + width,
            z_end: z + height,
            material,
        }
    }

    fn get_uv(&self, x: &f32, z: &f32) -> UvCoords {
        let u = (x - self.x_start) / (self.x_end - self.x_start);
        let v = (z - self.z_start) / (self.z_end - self.z_start);
        UvCoords { u, v }
    }
}

impl RayHitTester for PlaneY {
    fn hit(
        &self,
        ray: &crate::raytracing::ray::Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<HitResult> {
        let depth = (self.y - ray.origin.y()) / ray.direction.y();
        if depth < min_distance || depth > max_distance {
            return None;
        }

        let z = ray.origin.z() + depth * ray.direction.z();
        let x = ray.origin.x() + depth * ray.direction.x();
        if x < self.x_start || x > self.x_end || z < self.z_start || z > self.z_end {
            return None;
        }

        let location = ray.at(depth);
        let mut normal = Vec3::new(0., 1., 0.);
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
            uv: self.get_uv(&x, &z),
        })
    }
}

impl HittableObject for PlaneY {}

impl BoundingBox for PlaneY {
    fn bounding_box(
        &self,
        _: f32,
        _: f32,
    ) -> Result<crate::raytracing::aabb::AABB, crate::raytracing::aabb::BoundingBoxError> {
        todo!()
    }
}
// -------------------------------
