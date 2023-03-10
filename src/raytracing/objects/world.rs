use std::sync::Arc;

use crate::raytracing::{
    aabb::{BoundingBox, BoundingBoxError, AABB},
    ray::Ray,
    ray_hit::{HitResult, RayHitTester},
};

use super::HittableObject;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn HittableObject + Send + Sync>>,
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn HittableObject + Send + Sync>>) -> Self {
        Self { objects }
    }
}

impl HittableObject for HittableList {}

impl RayHitTester for HittableList {
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitResult> {
        let mut closest = max_distance;
        let mut temp_hit_result = None;

        for obj in &self.objects {
            if let Some(hit_result) = obj.hit(ray, min_distance, closest) {
                closest = hit_result.distance;
                temp_hit_result = Some(hit_result);
            }
        }
        temp_hit_result
    }
}

impl BoundingBox for HittableList {
    fn bounding_box(&self, start_time: f32, end_time: f32) -> Result<AABB, BoundingBoxError> {
        if self.objects.is_empty() {
            return Err(BoundingBoxError);
        }

        let mut temp_box = None;
        for obj in &self.objects {
            if let Ok(bb) = obj.bounding_box(start_time, end_time) {
                temp_box = if let Some(tb) = temp_box {
                    Some(<Self as BoundingBox>::surrounding_box(&bb, &tb))
                } else {
                    Some(bb)
                };
            } else {
                return Err(BoundingBoxError);
            }
        }
        temp_box.ok_or(BoundingBoxError)
    }
}
