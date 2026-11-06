//! Basic mathematical types and operations.

use std::ops::{Add, Div, Mul, Neg, Sub};

/// A two-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Creates a new vector.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns a zero vector.
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Returns the squared magnitude.
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the magnitude.
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns the dot product.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the distance between two vectors.
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Returns a normalized vector, or None for a zero vector.
    pub fn normalized(self) -> Option<Self> {
        let length = self.length();

        if length == 0.0 {
            None
        } else {
            Some(self / length)
        }
    }

    /// Multiplies the vector by a scalar.
    pub fn scale(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        self.scale(scalar)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

/// A three-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Creates a new vector.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns a zero vector.
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Returns the squared magnitude.
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the magnitude.
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns the dot product.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product.
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns the distance between two vectors.
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Returns a normalized vector, or None for a zero vector.
    pub fn normalized(self) -> Option<Self> {
        let length = self.length();

        if length == 0.0 {
            None
        } else {
            Some(self / length)
        }
    }

    /// Multiplies the vector by a scalar.
    pub fn scale(self, scalar: f32) -> Self {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        self.scale(scalar)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_creation_and_zero() {
        assert_eq!(Vec2::new(3.0, 4.0).x, 3.0);
        assert_eq!(Vec2::zero(), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn vec2_length() {
        assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0);
    }

    #[test]
    fn vec2_addition_and_subtraction() {
        let a = Vec2::new(2.0, 3.0);
        let b = Vec2::new(4.0, 5.0);

        assert_eq!(a + b, Vec2::new(6.0, 8.0));
        assert_eq!(b - a, Vec2::new(2.0, 2.0));
    }

    #[test]
    fn vec2_dot_product() {
        assert_eq!(
            Vec2::new(2.0, 3.0).dot(Vec2::new(4.0, 5.0)),
            23.0
        );
    }

    #[test]
    fn vec2_normalization() {
        let result = Vec2::new(3.0, 4.0).normalized().unwrap();

        assert!((result.x - 0.6).abs() < 0.0001);
        assert!((result.y - 0.8).abs() < 0.0001);
    }

    #[test]
    fn vec2_zero_normalization() {
        assert_eq!(Vec2::zero().normalized(), None);
    }

    #[test]
    fn vec3_creation_and_zero() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).z, 3.0);
        assert_eq!(Vec3::zero(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_length() {
        assert_eq!(Vec3::new(2.0, 3.0, 6.0).length(), 7.0);
    }

    #[test]
    fn vec3_addition_and_subtraction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(b - a, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn vec3_dot_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(a.dot(b), 32.0);
    }

    #[test]
    fn vec3_cross_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);

        assert_eq!(a.cross(b), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vec3_normalization() {
        let result = Vec3::new(2.0, 3.0, 6.0)
            .normalized()
            .unwrap();

        assert!((result.length() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn vec3_zero_normalization() {
        assert_eq!(Vec3::zero().normalized(), None);
    }

    #[test]
    fn vec3_distance() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0)
                .distance(Vec3::new(4.0, 6.0, 3.0)),
            5.0
        );
    }
}
