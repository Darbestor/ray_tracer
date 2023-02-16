use crate::math::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Move forward by 't' from 'origin', along the ray specified by 'direction'  
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }
}
