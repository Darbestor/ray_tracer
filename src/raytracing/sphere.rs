use std::{f32::consts::PI, sync::Arc};

use crate::math::vec3::Vec3;

use super::{
    aabb::{BoundingBox, BoundingBoxError, AABB},
    hittable::Hittable,
    material::Material,
    ray::Ray,
    ray_hit::{HitResult, Normal, RayHitTester},
    texture::{UvCoords, UvMapper},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {}

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
        let oc = ray.origin - self.center;
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

impl Normal for Sphere {
    fn get_normal(&self, location: &Vec3, _: &Ray) -> Vec3 {
        &(location - &self.center) / self.radius
    }
}

impl BoundingBox for Sphere {
    fn bounding_box(&self, _: f32, _: f32) -> Result<super::aabb::AABB, BoundingBoxError> {
        Ok(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

impl UvMapper for Sphere {
    fn get_uv_coords(&self, normal: &Vec3) -> super::texture::UvCoords {
        // normal: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f32::acos(-normal.y());
        let phi = f32::atan2(-normal.z(), normal.x()) + PI;
        UvCoords {
            u: phi / (2. * PI),
            v: theta / PI,
        }
    }
}
