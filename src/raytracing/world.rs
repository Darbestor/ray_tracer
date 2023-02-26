use super::{
    aabb::BoundingBox,
    hittable::Hittable,
    ray_hit::{HitResult, RayHitTester},
};

#[derive(Default)]
pub struct WorldObjects {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
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

impl BoundingBox for WorldObjects {
    fn bounding_box(&self, start_time: f32, end_time: f32) -> Option<super::aabb::AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut temp_box = None;
        for obj in &self.objects {
            if let Some(bb) = obj.bounding_box(start_time, end_time) {
                temp_box = if let Some(tb) = temp_box {
                    Some(<Self as BoundingBox>::surrounding_box(&bb, &tb))
                } else {
                    Some(bb)
                };
            } else {
                return None;
            }
        }
        temp_box
    }
}
