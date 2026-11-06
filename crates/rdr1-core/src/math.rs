//! Basic mathematical types and operations.

/// A two-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Creates a new vector.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the vector's magnitude.
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the dot product of two vectors.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

/// A three-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
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

    /// Returns the vector's magnitude.
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns the dot product of two vectors.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product of two vectors.
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_vector() {
        let vector = Vec2::new(3.0, 4.0);

        assert_eq!(vector.x, 3.0);
        assert_eq!(vector.y, 4.0);
    }

    #[test]
    fn calculates_length() {
        let vector = Vec2::new(3.0, 4.0);

        assert_eq!(vector.length(), 5.0);
    }

    #[test]
    fn calculates_dot_product() {
        let first = Vec2::new(2.0, 3.0);
        let second = Vec2::new(4.0, 5.0);

        assert_eq!(first.dot(second), 23.0);
    }
    
    #[test]
    fn creates_vec3() {
        let vector = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
    }

    #[test]
    fn calculates_vec3_length() {
        let vector = Vec3::new(2.0, 3.0, 6.0);

        assert_eq!(vector.length(), 7.0);
    }

    #[test]
    fn calculates_vec3_cross_product() {
        let first = Vec3::new(1.0, 0.0, 0.0);
        let second = Vec3::new(0.0, 1.0, 0.0);

        assert_eq!(
            first.cross(second),
            Vec3::new(0.0, 0.0, 1.0)
        );
    }
}
