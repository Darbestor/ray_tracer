use rand::{thread_rng, Rng};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{math::vec3::Vec3, utils::progress_watcher::ProgressObserver};

use super::{camera::Camera, objects::HittableObject, ray::Ray};

pub struct Renderer {
    pub camera: Camera,
    pub samples_per_pixel: usize,
    pub max_ray_bounces: usize,
    pub background: Vec3,
    pub objects: Box<dyn HittableObject + Send + Sync>,
}

impl Renderer {
    pub fn init(
        camera: Camera,
        samples_per_pixel: usize,
        max_ray_bounces: usize,
        objects: Box<dyn HittableObject + Send + Sync>,
    ) -> Self {
        Self {
            camera,
            samples_per_pixel,
            max_ray_bounces,
            objects,
            background: Vec3::new(0., 0., 0.),
        }
    }

    /// Render scene using
    pub fn render(&self, width: usize, height: usize, show_progress: bool) -> Vec<Vec3> {
        let mut pixels = vec![Vec3::zero(); width * height];

        let progress_bar = if show_progress {
            Some(ProgressObserver::new(width * height).start())
        } else {
            None
        };
        pixels.par_iter_mut().enumerate().for_each(|(p_ix, pixel)| {
            let mut rng = thread_rng();
            for _ in 0..self.samples_per_pixel {
                let x = ((p_ix % width) as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                let y = ((height - p_ix / width) as f32 + rng.gen::<f32>()) / (height - 1) as f32;

                let ray = self.camera.get_ray(x, y);
                *pixel += self.render_pixel(&ray, self.max_ray_bounces);
            }

            let scale = 1. / self.samples_per_pixel as f32;
            pixel.set_x(f32::sqrt(pixel.x() * scale));
            pixel.set_y(f32::sqrt(pixel.y() * scale));
            pixel.set_z(f32::sqrt(pixel.z() * scale));

            // Update progress
            if show_progress {
                progress_bar.as_ref().unwrap().increase(1);
            }
        });

        pixels
    }

    fn render_pixel(&self, ray: &Ray, depth: usize) -> Vec3 {
        if depth == 0 {
            return Vec3::zero();
        }

        if let Some(hit) = self.objects.hit(ray, 0.001, f32::INFINITY) {
            let emitted = hit.material.emitted(&hit.uv, &hit.location);
            if let Some(scatter_result) = hit.material.scatter(ray, &hit) {
                emitted
                    + scatter_result.attenuation * self.render_pixel(&scatter_result.ray, depth - 1)
            } else {
                emitted
            }
        } else {
            self.background
        }

        // let unit_direction = ray.direction.norm();
        // let t = 0.5 * (unit_direction.y() + 1.0);
        // // Blend between white and blue
        // (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
