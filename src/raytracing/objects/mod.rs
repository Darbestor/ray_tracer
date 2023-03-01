use super::{aabb::BoundingBox, ray_hit::RayHitTester};

pub mod bvh;
pub mod moving_sphere;
pub mod plane;
pub mod sphere;
pub mod world;

pub use bvh::BvhNode;
pub use moving_sphere::MovingSphere;
pub use plane::Plane;
pub use sphere::Sphere;
pub use world::WorldObjects;

pub trait HittableObject: RayHitTester + BoundingBox {}
