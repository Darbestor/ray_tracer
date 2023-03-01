use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::raytracing::{
    aabb::{BoundingBox, BoundingBoxError, AABB},
    ray_hit::RayHitTester,
};

use super::HittableObject;

pub struct BvhNode {
    pub left: Arc<dyn HittableObject + Send + Sync>,
    pub right: Arc<dyn HittableObject + Send + Sync>,
    bounding_box: AABB,
}

impl BvhNode {
    pub fn new(
        objects: &[Arc<dyn HittableObject + Send + Sync>],
        start_time: f32,
        end_time: f32,
    ) -> Result<Self, BoundingBoxError> {
        BvhNode::construct(objects, 0, objects.len(), start_time, end_time)
    }

    fn construct(
        objects: &[Arc<dyn HittableObject + Send + Sync>],
        start: usize,
        end: usize,
        start_time: f32,
        end_time: f32,
    ) -> Result<Self, BoundingBoxError> {
        let mut objects: Vec<Arc<dyn HittableObject + Send + Sync>> = objects.to_vec();
        let axis: usize = thread_rng().gen_range(0..3);
        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if Self::comparator(&objects[start], &objects[start + 1], axis)
                == std::cmp::Ordering::Less
            {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects.sort_unstable_by(|left, right| Self::comparator(left, right, axis));

            let mid = start + object_span / 2;
            let (left, right) = (
                Self::construct(&objects, start, mid, start_time, end_time)?,
                Self::construct(&objects, mid, end, start_time, end_time)?,
            );
            let left: Arc<dyn HittableObject + Send + Sync> = Arc::new(left);
            let right: Arc<dyn HittableObject + Send + Sync> = Arc::new(right);
            (left, right)
        };

        let box_left = left.bounding_box(start_time, end_time)?;
        let box_right = right.bounding_box(start_time, end_time)?;

        let bounding_box = <BvhNode as BoundingBox>::surrounding_box(&box_left, &box_right);
        Ok(Self {
            left,
            right,
            bounding_box,
        })
    }

    fn comparator(
        left: &Arc<dyn HittableObject + Send + Sync>,
        right: &Arc<dyn HittableObject + Send + Sync>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let left_aabb = left.bounding_box(0., 0.);
        let right_aabb = right.bounding_box(0., 0.);
        match (left_aabb, right_aabb) {
            (Ok(left), Ok(right)) => left.minimum[axis].total_cmp(&right.minimum[axis]),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl HittableObject for BvhNode {}

impl BoundingBox for BvhNode {
    fn bounding_box(&self, _: f32, _: f32) -> Result<AABB, BoundingBoxError> {
        Ok(self.bounding_box)
    }
}

impl RayHitTester for BvhNode {
    fn hit(
        &self,
        ray: &crate::raytracing::ray::Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<crate::raytracing::ray_hit::HitResult> {
        self.bounding_box.hit(ray)?;

        let left = self.left.hit(ray, min_distance, max_distance).map(|hit| {
            if let Some(hit2) = self.right.hit(ray, min_distance, hit.distance) {
                hit2
            } else {
                hit
            }
        });
        left.or_else(|| self.right.hit(ray, min_distance, max_distance))
    }
}
