use crate::{
    math::{degrees_to_radians, vec3::Vec3},
    raytracing::{
        aabb::{BoundingBox, BoundingBoxError, AABB},
        ray::Ray,
        ray_hit::{HitResult, RayHitTester},
    },
};

use super::HittableObject;

pub struct YawRotation {
    instance: Box<dyn HittableObject + Send + Sync>,
    sin_theta: f32,
    cos_theta: f32,
    aabb: Result<AABB, BoundingBoxError>,
}

impl YawRotation {
    pub fn new(instance: Box<dyn HittableObject + Send + Sync>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let (sin_theta, cos_theta) = f32::sin_cos(radians);

        let aabb = instance.bounding_box(0., 1.).map(|aabb| {
            let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
            let mut max = Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

            for i in 0..2 {
                let i = i as f32;
                for j in 0..2 {
                    let j = j as f32;
                    for k in 0..2 {
                        let k = k as f32;
                        let x = i * aabb.maximum.x() + (1. - i) * aabb.minimum.x();
                        let y = j * aabb.maximum.y() + (1. - j) * aabb.minimum.y();
                        let z = k * aabb.maximum.z() + (1. - k) * aabb.minimum.z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        min.set_x(f32::min(min.x(), tester[0]));
                        min.set_y(f32::min(min.y(), tester[1]));
                        min.set_z(f32::min(min.z(), tester[2]));

                        max.set_x(f32::max(max.x(), tester[0]));
                        max.set_y(f32::max(max.y(), tester[1]));
                        max.set_z(f32::max(max.z(), tester[2]));
                    }
                }
            }

            AABB::new(min, max)
        });
        Self {
            instance,
            sin_theta,
            cos_theta,
            aabb,
        }
    }
}

impl HittableObject for YawRotation {}

impl BoundingBox for YawRotation {
    fn bounding_box(&self, _: f32, _: f32) -> Result<AABB, BoundingBoxError> {
        match self.aabb.as_ref() {
            Ok(bb) => Ok(AABB::new(bb.minimum, bb.maximum)),
            Err(_) => Err(BoundingBoxError),
        }
    }
}

impl RayHitTester for YawRotation {
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitResult> {
        let mut new_origin = ray.origin;
        let mut new_direction = ray.direction;

        let origin = ray.origin;
        let direction = ray.direction;

        new_origin.set_x(self.cos_theta * origin.x() - self.sin_theta * origin.z());
        new_origin.set_z(self.sin_theta * origin.x() + self.cos_theta * origin.z());

        new_direction.set_x(self.cos_theta * direction.x() - self.sin_theta * direction.z());
        new_direction.set_z(self.sin_theta * direction.x() + self.cos_theta * direction.z());
        let ray_rotated = Ray::new(new_origin, new_direction, ray.time);

        self.instance
            .hit(&ray_rotated, min_distance, max_distance)
            .map(|mut hit| {
                let location = hit.location;
                let normal = hit.normal;

                hit.location
                    .set_x(self.cos_theta * location.x() + self.sin_theta * location.z());
                hit.location
                    .set_z((-self.sin_theta) * location.x() + self.cos_theta * location.z());

                hit.normal
                    .set_x(self.cos_theta * normal.x() + self.sin_theta * normal.z());

                hit.normal
                    .set_z((-self.sin_theta) * normal.x() + self.cos_theta * normal.z());

                let front_face = ray_rotated.direction.dot(&hit.normal) < 0.;
                if !front_face {
                    hit.normal = -hit.normal;
                }
                hit.front_face = front_face;

                hit
            })
    }
}
