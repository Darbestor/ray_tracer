use crate::math::vec3::Vec3;

use super::{ray::Ray, ray_hit::RayHitTester};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl RayHitTester for Sphere {
    /** If equation `t^2*b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0` has roots
    then [Ray](Ray) hits sphere.

    1 root - hit circumference.
    2 roots - pass through sphere.

    `t` - distance from camera to hit point

    `b` - [Ray](Ray) direction

    `A` - [Ray](Ray) origin

    `C` - [Sphere](Sphere) center
    */
    fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - &self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;
        discriminant.is_sign_positive()
    }
}
