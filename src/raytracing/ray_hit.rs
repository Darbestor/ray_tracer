use std::rc::Rc;

use crate::math::vec3::Vec3;

use super::{material::Material, ray::Ray};

pub struct HitResult {
    pub location: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool,
    pub material: Rc<Material>,
}

impl HitResult {
    pub fn new<T: Normal>(object: &T, ray: &Ray, distance: f32, material: Rc<Material>) -> Self {
        let location = ray.at(distance);
        let mut normal = object.get_normal(&location);
        let front_face = ray.direction.dot(&normal) < 0.;
        if !front_face {
            normal = -normal;
        }
        Self {
            location,
            normal,
            distance,
            front_face,
            material,
        }
    }
}

pub trait Normal {
    fn get_normal(&self, location: &Vec3) -> Vec3;
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
