use crate::math::vec3::Vec3;

use super::ray::Ray;

/// Test on ray intersetion with object
pub trait RayHitTester {
    /// Trace and find point where [ray](Ray) hits the object
    ///
    /// ## Returns
    ///
    /// If ray hits the object, returns distance from `ray` `origin` to hit location
    fn hit(&self, ray: &Ray) -> Option<f32>;

    /// Checks if [ray](Ray) hits the object
    fn is_hit(&self, ray: &Ray) -> bool {
        self.hit(ray).is_some()
    }
}
