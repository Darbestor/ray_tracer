use crate::math::vec3::Vec3;

use super::{
    ray::Ray,
    ray_hit::{HitResult, Normal, RayHitTester},
};

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
    /** [`Ray`] hit test for sphere

    ## Returns

    Closest distance to shere's hit location if found

    ### Calculations
    If equation `t^2*b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0` has roots
    then [Ray](Ray) hits sphere.

    1 root - hit circumference.
    2 roots - pass through sphere.
    `t` - distance from camera to hit point

    `b` - [Ray](Ray) direction

    `A` - [Ray](Ray) origin

    `C` - [Sphere](Sphere) center
    */
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitResult> {
        let oc = ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            None
        }
        // Find the nearest root that lies in the acceptable range.
        else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < min_distance || max_distance < root {
                root = (-half_b + sqrtd) / a;
                if root < min_distance || max_distance < root {
                    return None;
                }
            }
            Some(HitResult::new(self, ray, root))
        }
    }
}

impl Normal for Sphere {
    fn get_normal(&self, location: &Vec3) -> Vec3 {
        location - &(&self.center / self.radius)
    }
}
