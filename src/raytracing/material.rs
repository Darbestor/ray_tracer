/// TODO: Think about different design structure
use crate::math::{random_in_unit_sphere, vec3::Vec3};

use super::{ray::Ray, ray_hit::HitResult};

pub enum Material {
    Labmertian(MatLabmertian),
    Metalic(MatMetalic),
    Dielectric(MatDielectric),
}

pub struct MatLabmertian {
    pub albedo: Vec3,
}

pub struct MatMetalic {
    pub albedo: Vec3,
    pub roughness: f32,
}

pub struct MatDielectric {
    pub refraction_index: f32,
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
            Material::Dielectric(mat) => mat.scatter(ray, hit_result),
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
    pub fn new(albedo: Vec3, roughness: f32) -> Self {
        Self {
            albedo,
            roughness: roughness.clamp(-1., 1.),
        }
    }
}

impl MaterialScatter for MatMetalic {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let reflection = MaterialFunctions::reflect(&ray.direction, &hit_result.normal);
        let scattered = Ray::new(
            hit_result.location,
            reflection + self.roughness * random_in_unit_sphere(),
        );

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

impl MatDielectric {
    const ALBEDO: Vec3 = Vec3::new(1., 1., 1.);
}

impl MaterialScatter for MatDielectric {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let refraction_ratio = if hit_result.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let refracted =
            MaterialFunctions::refract(&ray.direction.norm(), &hit_result.normal, refraction_ratio);
        Some(ScatterResult {
            attenuation: MatDielectric::ALBEDO,
            ray: Ray::new(hit_result.location, refracted),
        })
    }
}

struct MaterialFunctions;

impl MaterialFunctions {
    /// `normal` must be normalized
    fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
        *v - 2. * v.dot(normal) * normal
    }

    fn refract(v: &Vec3, normal: &Vec3, refraction_ratio: f32) -> Vec3 {
        let cos_theta = f32::min((-v).dot(normal), 1.);
        let sin_theta = f32::sqrt(1. - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract {
            MaterialFunctions::reflect(v, normal)
        } else {
            let ray_out_perpendicular = refraction_ratio * (v + &(cos_theta * normal));
            let ray_out_parallel =
                -f32::sqrt(f32::abs(1.0 - ray_out_perpendicular.length_squared())) * normal;
            ray_out_parallel + ray_out_perpendicular
        }
    }
}
