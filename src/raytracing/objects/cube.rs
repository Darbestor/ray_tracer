use std::sync::Arc;

use crate::{
    math::vec3::Vec3,
    raytracing::{
        aabb::{BoundingBox, AABB},
        material::Material,
        ray_hit::RayHitTester,
    },
};

use super::{HittableList, HittableObject, PlaneX, PlaneY, PlaneZ};

pub struct Cube {
    hittable_list: HittableList,
    min_point: Vec3,
    max_point: Vec3,
    pub material: Arc<Material>,
}

impl Cube {
    pub fn new(min_point: Vec3, max_point: Vec3, material: Arc<Material>) -> Self {
        let bottom = Arc::new(PlaneY::new(
            min_point.x(),
            min_point.y(),
            min_point.z(),
            max_point.x() - min_point.x(),
            max_point.z() - min_point.z(),
            material.clone(),
        ));
        let top = Arc::new(PlaneY::new(
            min_point.x(),
            max_point.y(),
            min_point.z(),
            max_point.x() - min_point.x(),
            max_point.z() - min_point.z(),
            material.clone(),
        ));
        let left = Arc::new(PlaneX::new(
            min_point.x(),
            min_point.y(),
            min_point.z(),
            max_point.y() - min_point.y(),
            max_point.z() - min_point.z(),
            material.clone(),
        ));
        let right = Arc::new(PlaneX::new(
            max_point.x(),
            min_point.y(),
            min_point.z(),
            max_point.y() - min_point.y(),
            max_point.z() - min_point.z(),
            material.clone(),
        ));
        let front = Arc::new(PlaneZ::new(
            min_point.x(),
            min_point.y(),
            min_point.z(),
            max_point.x() - min_point.x(),
            max_point.y() - min_point.y(),
            material.clone(),
        ));
        let back = Arc::new(PlaneZ::new(
            min_point.x(),
            min_point.y(),
            max_point.z(),
            max_point.x() - min_point.x(),
            max_point.y() - min_point.y(),
            material.clone(),
        ));

        Self {
            hittable_list: HittableList::new(vec![top, bottom, left, right, front, back]),
            min_point,
            max_point,
            material,
        }
    }
}

impl RayHitTester for Cube {
    fn hit(
        &self,
        ray: &crate::raytracing::ray::Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<crate::raytracing::ray_hit::HitResult> {
        self.hittable_list.hit(ray, min_distance, max_distance)
    }
}

impl HittableObject for Cube {}

impl BoundingBox for Cube {
    fn bounding_box(
        &self,
        _: f32,
        _: f32,
    ) -> Result<AABB, crate::raytracing::aabb::BoundingBoxError> {
        Ok(AABB::new(self.min_point, self.max_point))
    }
}
