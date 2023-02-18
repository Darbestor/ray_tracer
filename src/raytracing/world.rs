use super::ray_hit::{HitResult, RayHitTester};

#[derive(Default)]
pub struct WorldObjects {
    pub objects: Vec<Box<dyn RayHitTester + Send + Sync>>,
}

impl WorldObjects {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}

impl RayHitTester for WorldObjects {
    fn hit(
        &self,
        ray: &super::ray::Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<HitResult> {
        let mut closest = max_distance;
        let mut temp_hit_result = None;

        for obj in &self.objects {
            if let Some(hit_result) = obj.hit(ray, min_distance, closest) {
                closest = hit_result.distance;
                temp_hit_result = Some(hit_result);
            }
        }
        temp_hit_result
    }
}
