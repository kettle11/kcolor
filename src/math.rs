// Select snippets from kettlemath extracted for use in this library and made to use f64.

use std::ops::{Mul, Sub};
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn dot(a: Vector3, b: Vector3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[inline]
    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        (a.zxy() * b - a * b.zxy()).zxy()
    }

    #[inline]
    pub fn zxy(self) -> Vector3 {
        Vector3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Matrix3x3 {
    c0: Vector3,
    c1: Vector3,
    c2: Vector3,
}

impl Matrix3x3 {
    pub fn from_columns(c0: Vector3, c1: Vector3, c2: Vector3) -> Self {
        Matrix3x3 { c0, c1, c2 }
    }
    pub fn row0(&self) -> Vector3 {
        Vector3::new(self.c0.x, self.c1.x, self.c2.x)
    }

    pub fn row1(&self) -> Vector3 {
        Vector3::new(self.c0.y, self.c1.y, self.c2.y)
    }

    pub fn row2(&self) -> Vector3 {
        Vector3::new(self.c0.z, self.c1.z, self.c2.z)
    }

    pub fn determinant(&self) -> f64 {
        Vector3::dot(self.c2, Vector3::cross(self.c0, self.c1))
    }

    // Unverified, code hasn't event been run.
    pub fn inverse(&self) -> Matrix3x3 {
        let inverse_determinant = 1.0 / self.determinant();

        let m00 = self.c0.x;
        let m01 = self.c0.y;
        let m02 = self.c0.z;
        let m10 = self.c1.x;
        let m11 = self.c1.y;
        let m12 = self.c1.z;
        let m20 = self.c2.x;
        let m21 = self.c2.y;
        let m22 = self.c2.z;

        let i00 = (m11 * m22 - m21 * m12) * inverse_determinant;
        let i01 = (m02 * m21 - m01 * m22) * inverse_determinant;
        let i02 = (m01 * m12 - m02 * m11) * inverse_determinant;
        let i10 = (m12 * m20 - m10 * m22) * inverse_determinant;
        let i11 = (m00 * m22 - m02 * m20) * inverse_determinant;
        let i12 = (m10 * m02 - m00 * m12) * inverse_determinant;
        let i20 = (m10 * m21 - m20 * m11) * inverse_determinant;
        let i21 = (m20 * m01 - m00 * m21) * inverse_determinant;
        let i22 = (m00 * m11 - m10 * m01) * inverse_determinant;

        Matrix3x3 {
            c0: Vector3::new(i00, i01, i02),
            c1: Vector3::new(i10, i11, i12),
            c2: Vector3::new(i20, i21, i22),
        }
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;
    #[inline]
    fn mul(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: Vector3::dot(self.row0(), other),
            y: Vector3::dot(self.row1(), other),
            z: Vector3::dot(self.row2(), other),
        }
    }
}
