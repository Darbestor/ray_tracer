use std::mem::swap;

use thiserror::Error;

use crate::math::vec3::Vec3;

use super::ray::Ray;

#[derive(Error, Debug)]
#[error("Bounding box is not supported for the type")]
pub struct BoundingBoxError;

pub trait BoundingBox {
    fn bounding_box(&self, start_time: f32, end_time: f32) -> Result<AABB, BoundingBoxError>;

    fn surrounding_box(box1: &AABB, box2: &AABB) -> AABB
    where
        Self: Sized,
    {
        let small = (
            f32::min(box1.minimum.x(), box2.minimum.x()),
            f32::min(box1.minimum.y(), box2.minimum.y()),
            f32::min(box1.minimum.z(), box2.minimum.z()),
        );

        let big = (
            f32::max(box1.maximum.x(), box2.maximum.x()),
            f32::max(box1.maximum.y(), box2.maximum.y()),
            f32::max(box1.maximum.z(), box2.maximum.z()),
        );
        AABB::new(
            Vec3::new(small.0, small.1, small.2),
            Vec3::new(big.0, big.1, big.2),
        )
    }
}

#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

pub struct AabbIntersectionInterval(f32, f32);

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray) -> Option<AabbIntersectionInterval> {
        let mut interval = AabbIntersectionInterval(0., 0.);
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut start = (self.minimum[i] - ray.origin[i]) * inv_d;
            let mut end = (self.maximum[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut start, &mut end);
            }
            if start > interval.0 {
                interval.0 = start;
            }
            if end > interval.1 {
                interval.1 = end;
            }
            if interval.1 <= interval.0 {
                return None;
            }
        }
        Some(interval)
    }
}
