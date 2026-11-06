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
}
