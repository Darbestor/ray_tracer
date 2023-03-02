use crate::{
    math::vec3::Vec3,
    raytracing::{
        aabb::{BoundingBox, BoundingBoxError, AABB},
        ray::Ray,
        ray_hit::{HitResult, RayHitTester},
    },
};

use super::HittableObject;

pub struct Translate {
    instance: Box<dyn HittableObject + Send + Sync>,
    offset: Vec3,
}

impl Translate {
    pub fn new(instance: Box<dyn HittableObject + Send + Sync>, displacement: Vec3) -> Self {
        Self {
            instance,
            offset: displacement,
        }
    }
}

impl HittableObject for Translate {}

impl BoundingBox for Translate {
    fn bounding_box(&self, start_time: f32, end_time: f32) -> Result<AABB, BoundingBoxError> {
        let aabb = self.instance.bounding_box(start_time, end_time)?;
        Ok(AABB::new(
            aabb.minimum + self.offset,
            aabb.maximum + self.offset,
        ))
    }
}

impl RayHitTester for Translate {
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitResult> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        self.instance
            .hit(&moved_ray, min_distance, max_distance)
            .map(|mut hit| {
                hit.location += self.offset;
                let front_face = moved_ray.direction.dot(&hit.normal) < 0.;
                if !front_face {
                    hit.normal = -hit.normal;
                }
                hit.front_face = front_face;
                hit
            })
    }
}
