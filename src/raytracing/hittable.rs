use super::{aabb::BoundingBox, ray_hit::RayHitTester};

pub trait Hittable: RayHitTester + BoundingBox {}
