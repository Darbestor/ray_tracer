use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

/// 3-Dimensional vector
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    values: [f32; 3],
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { values: [x, y, z] }
    }

    pub const fn zero() -> Self {
        Vec3 {
            values: [0., 0., 0.],
        }
    }

    pub const fn x(&self) -> f32 {
        self.values[0]
    }

    pub const fn y(&self) -> f32 {
        self.values[1]
    }

    pub const fn z(&self) -> f32 {
        self.values[2]
    }

    /// Vector length (magnitude)
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    /// Squared length in each direction
    pub fn length_squared(&self) -> f32 {
        self.values.iter().fold(0., |acc, v| acc + (v * v))
    }

    /// Dot product
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.values
            .iter()
            .zip(rhs.values)
            .fold(0., |acc, (v1, v2)| acc + v1 * v2)
    }

    /// Cross product
    pub fn cross(&self, rhs: &Self) -> Self {
        let lvec = &self.values;
        let rvec = &rhs.values;
        Self {
            values: [
                lvec[1] * rvec[2] - lvec[2] * rvec[1],
                lvec[2] * rvec[0] - lvec[0] * rvec[2],
                lvec[0] * rvec[1] - lvec[1] * rvec[0],
            ],
        }
    }

    /// Normalized vector's length
    pub fn unit(&self) -> Self {
        self / self.length()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "x:{} y:{} z:{}", self.x(), self.y(), self.z())
    }
}

/// Assign operations overrides
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.values[0] += rhs.x();
        self.values[1] += rhs.y();
        self.values[2] += rhs.z();
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.values.iter_mut().for_each(|v| *v *= rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.mul_assign(1. / rhs);
    }
}
//////////////////

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            values: self.values.map(|v| -v),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &'b Vec3) -> Self::Output {
        Vec3 {
            values: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &'b Vec3) -> Self::Output {
        Vec3 {
            values: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            values: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            values: self.values.map(|v| v * rhs),
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        (&rhs).mul(self)
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        self.mul(1. / rhs)
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn length_test() {
        let vec = Vec3::new(0., 0., 0.);
        assert_eq!(vec.length(), 0.);

        let vec = Vec3::new(1., 2., 3.);
        assert!(f32::abs(vec.length() - 3.741_657_5) < f32::EPSILON);
    }

    #[test]
    fn dot_test() {
        let v1 = Vec3::new(0., 0., 0.);
        assert_eq!(v1.dot(&v1), 0.);

        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(10., 20., 30.);
        assert_eq!(v1.dot(&v2), 140.);
        assert_eq!(v2.dot(&v1), 140.);
    }

    #[test]
    fn cross_test() {
        let v1 = Vec3::new(0., 0., 0.);
        let res = v1.cross(&v1);
        assert_eq!(res.x(), 0.);
        assert_eq!(res.y(), 0.);
        assert_eq!(res.z(), 0.);

        let v1 = Vec3::new(1., 1., 1.);
        let v2 = Vec3::new(10., 20., 30.);
        let res = v1.cross(&v2);
        assert_eq!(res.x(), 10.);
        assert_eq!(res.y(), -20.);
        assert_eq!(res.z(), 10.);
        let res = v2.cross(&v1);
        assert_eq!(res.x(), -10.);
        assert_eq!(res.y(), 20.);
        assert_eq!(res.z(), -10.);
    }
}
