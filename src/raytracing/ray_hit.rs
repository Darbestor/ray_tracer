use std::sync::Arc;

use crate::math::vec3::Vec3;

use super::{
    material::Material,
    ray::Ray,
    texture::{UvCoords, UvMapper},
};

pub struct HitResult {
    pub location: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool,
    pub material: Arc<Material>,
    // texture coords on surface
    pub uv: UvCoords,
}

impl HitResult {
    pub fn new<T: Normal + UvMapper>(
        object: &T,
        ray: &Ray,
        distance: f32,
        material: Arc<Material>,
    ) -> Self {
        let location = ray.at(distance);
        let mut normal = object.get_normal(&location, ray);
        let front_face = ray.direction.dot(&normal) < 0.;
        if !front_face {
            normal = -normal;
        }
        let uv = object.get_uv_coords(&normal);
        Self {
            location,
            normal,
            distance,
            front_face,
            material,
            uv,
        }
    }
}

pub trait Normal {
    fn get_normal(&self, location: &Vec3, ray: &Ray) -> Vec3;
}

/// Test on ray intersetion with object
pub trait RayHitTester {
    /// Trace and find point where [ray](Ray) hits the object
    ///
    /// ## Returns
    ///
    /// If ray hits the object, returns distance from `ray` `origin` to hit location
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitResult>;
}
