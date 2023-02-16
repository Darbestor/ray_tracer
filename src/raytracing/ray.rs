use crate::math::vec3::Vec3;

pub struct Ray<'a> {
    pub origin: &'a Vec3,
    pub direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, direction: &'a Vec3) -> Self {
        Self { origin, direction }
    }

    /// Move forward by 't' from 'origin', along the ray specified by 'direction'  
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + &(t * self.direction)
    }
}
