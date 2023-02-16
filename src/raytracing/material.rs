/// TODO: Think about different design structure
use crate::math::{random_in_unit_sphere, vec3::Vec3};

use super::{ray::Ray, ray_hit::HitResult};

pub enum Material {
    Labmertian(MatLabmertian),
    Metalic(MatMetalic),
}

pub struct MatLabmertian {
    pub albedo: Vec3,
}

pub struct MatMetalic {
    pub albedo: Vec3,
}

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub trait MaterialScatter {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult>;
}

impl MaterialScatter for Material {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        match self {
            Material::Labmertian(mat) => mat.scatter(ray, hit_result),
            Material::Metalic(mat) => mat.scatter(ray, hit_result),
        }
    }
}

impl MaterialScatter for MatLabmertian {
    fn scatter(&self, _: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let scatter_direction =
            hit_result.location + hit_result.normal + random_in_unit_sphere().norm();
        let scattered = Ray::new(hit_result.location, scatter_direction - hit_result.location);
        Some(ScatterResult {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}

impl MatMetalic {
    /// `normal` must be normalized
    fn reflect(&self, v: &Vec3, normal: &Vec3) -> Vec3 {
        *v - 2. * v.dot(normal) * normal
    }
}

impl MaterialScatter for MatMetalic {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let reflection = self.reflect(&ray.direction, &hit_result.normal);
        let scattered = Ray::new(hit_result.location, reflection);

        if scattered.direction.dot(&hit_result.normal) > 0. {
            Some(ScatterResult {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            None
        }
    }
}
