use std::sync::Arc;

use crate::math::vec3::Vec3;

use super::{
    aabb::{BoundingBox, BoundingBoxError, AABB},
    hittable::Hittable,
    material::Material,
    ray::Ray,
    ray_hit::{HitResult, Normal, RayHitTester},
};

pub struct MovingSphere {
    pub center_start: Vec3,
    pub center_end: Vec3,
    pub time_start: f32,
    pub time_end: f32,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl MovingSphere {
    pub fn new(
        center_start: Vec3,
        center_end: Vec3,
        time_start: f32,
        time_end: f32,
        radius: f32,
        material: Arc<Material>,
    ) -> Self {
        Self {
            center_start,
            center_end,
            time_start,
            time_end,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

impl Hittable for MovingSphere {}

impl RayHitTester for MovingSphere {
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
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
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
            Some(HitResult::new(self, ray, root, self.material.clone()))
        }
    }
}

impl Normal for MovingSphere {
    fn get_normal(&self, location: &Vec3, ray: &Ray) -> Vec3 {
        &(location - &self.center(ray.time)) / self.radius
    }
}

impl BoundingBox for MovingSphere {
    fn bounding_box(
        &self,
        start_time: f32,
        end_time: f32,
    ) -> Result<super::aabb::AABB, BoundingBoxError> {
        let box_start = AABB::new(
            self.center(start_time) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(start_time) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box_end = AABB::new(
            self.center(end_time) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(end_time) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Ok(<MovingSphere as BoundingBox>::surrounding_box(
            &box_start, &box_end,
        ))
    }
}
