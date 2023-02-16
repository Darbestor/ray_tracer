use super::ray::Ray;

/// Test on ray intersetion with object
pub trait RayHitTester {
    /// Checks if [ray](Ray) hits the object
    fn hit(&self, ray: &Ray) -> bool;
}
