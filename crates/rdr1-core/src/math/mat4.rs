
use std::ops::Mul;

use super::Vec3;

/// A 4x4 matrix using row-major storage and column-vector multiplication.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub elements: [[f32; 4]; 4],
}

impl Mat4 {
    /// Creates a matrix from four rows.
    pub const fn new(elements: [[f32; 4]; 4]) -> Self {
        Self { elements }
    }

    /// Returns the identity matrix.
    pub const fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a translation matrix.
    pub const fn translation(offset: Vec3) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, offset.x],
            [0.0, 1.0, 0.0, offset.y],
            [0.0, 0.0, 1.0, offset.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a scaling matrix.
    pub const fn scaling(scale: Vec3) -> Self {
        Self::new([
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Transforms a point, including translation.
    pub fn transform_point(self, point: Vec3) -> Vec3 {
        let x = self.elements[0][0] * point.x
            + self.elements[0][1] * point.y
            + self.elements[0][2] * point.z
            + self.elements[0][3];

        let y = self.elements[1][0] * point.x
            + self.elements[1][1] * point.y
            + self.elements[1][2] * point.z
            + self.elements[1][3];

        let z = self.elements[2][0] * point.x
            + self.elements[2][1] * point.y
            + self.elements[2][2] * point.z
            + self.elements[2][3];

        let w = self.elements[3][0] * point.x
            + self.elements[3][1] * point.y
            + self.elements[3][2] * point.z
            + self.elements[3][3];

        if w == 0.0 || !w.is_finite() {
            return Vec3::new(f32::NAN, f32::NAN, f32::NAN);
        }

        Vec3::new(x / w, y / w, z / w)
    }

    /// Transforms a direction without applying translation.
    pub fn transform_vector(self, vector: Vec3) -> Vec3 {
        Vec3::new(
            self.elements[0][0] * vector.x
                + self.elements[0][1] * vector.y
                + self.elements[0][2] * vector.z,
            self.elements[1][0] * vector.x
                + self.elements[1][1] * vector.y
                + self.elements[1][2] * vector.z,
            self.elements[2][0] * vector.x
                + self.elements[2][1] * vector.y
                + self.elements[2][2] * vector.z,
        )
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = [[0.0; 4]; 4];

        for (row_index, row) in result.iter_mut().enumerate() {
            for (column_index, value) in row.iter_mut().enumerate() {
                for index in 0..4 {
                    *value += self.elements[row_index][index]
                        * other.elements[index][column_index];
                }
            }
        }

        Self::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix_preserves_point() {
        let point = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(Mat4::identity().transform_point(point), point);
    }

    #[test]
    fn default_is_identity() {
        assert_eq!(Mat4::default(), Mat4::identity());
    }

    #[test]
    fn translation_moves_point() {
        let matrix = Mat4::translation(Vec3::new(5.0, 2.0, 1.0));
        let point = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(
            matrix.transform_point(point),
            Vec3::new(6.0, 4.0, 4.0)
        );
    }

    #[test]
    fn translation_does_not_move_vector() {
        let matrix = Mat4::translation(Vec3::new(5.0, 2.0, 1.0));
        let vector = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(matrix.transform_vector(vector), vector);
    }

    #[test]
    fn scaling_changes_point() {
        let matrix = Mat4::scaling(Vec3::new(2.0, 3.0, 4.0));
        let point = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(
            matrix.transform_point(point),
            Vec3::new(2.0, 6.0, 12.0)
        );
    }

    #[test]
    fn matrix_multiplication_preserves_identity() {
        let matrix = Mat4::translation(Vec3::new(1.0, 2.0, 3.0));

        assert_eq!(matrix * Mat4::identity(), matrix);
        assert_eq!(Mat4::identity() * matrix, matrix);
    }

    #[test]
    fn matrix_multiplication_combines_transforms() {
        let translation = Mat4::translation(Vec3::new(1.0, 2.0, 3.0));
        let scaling = Mat4::scaling(Vec3::new(2.0, 2.0, 2.0));

        let combined = translation * scaling;
        let result = combined.transform_point(Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(result, Vec3::new(3.0, 4.0, 5.0));
    }
}
