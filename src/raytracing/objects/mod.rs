use super::{aabb::BoundingBox, ray_hit::RayHitTester};

pub mod bvh;
pub mod cube;
pub mod moving_sphere;
pub mod plane;
pub mod sphere;
pub mod translate;
pub mod world;

pub use bvh::BvhNode;
pub use cube::Cube;
pub use moving_sphere::MovingSphere;
pub use plane::PlaneX;
pub use plane::PlaneY;
pub use plane::PlaneZ;
pub use sphere::Sphere;
pub use translate::Translate;
pub use world::HittableList;

pub trait HittableObject: RayHitTester + BoundingBox {}
