/*
Some math libs to take inspiration from:

https://crates.io/crates/cgmath
https://github.com/rustsim (NAlgebra, NPhysics, NCollide, Alga)
https://crates.io/crates/num

And there's a bunch of Geometric Algebra libs out there, but then
first we need a basic tour of how to actually use GA. (More reading)

For now, let's create only the types we need.

Todo:
- Impl approx_eq for testing https://docs.rs/approx/0.3.2/approx/

Todo much later:
- Specification of generic Scalar type trait bound
- Write vector arithmetic such that number of dimensions is a generic argument
- partialEq, Eq and Hash all don't work with f32

*/

use std::ops::*;

/*--------------------
    Vec2f
--------------------*/ 

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2f {
            x: x,
            y: y
        }
    }

    pub fn dot(a : Self, b : Self) -> f32 {
        a.x * b.y + a.y * b.y
    }
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2f {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2f{
    
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2f {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vec2f {
    type Output = Self;

    fn mul(self, other: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, other: f32) -> Vec2f {
        Vec2f {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div for Vec2f {
    type Output = Self;

    fn div(self, other: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f32> for Vec2f {
    type Output = Self;

    fn div(self, other: f32) -> Vec2f {
        Vec2f {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Index<usize> for Vec2f {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &0.0
        }
    }
}

/*--------------------
    Vec3f
--------------------*/ 

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f {
            x: x,
            y: y,
            z: z,
        }
    }

    // Todo: define using inner product trait?
    pub fn dot(a : Vec3f, b : Vec3f) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a : Vec3f, b : Vec3f) -> Self {
        Vec3f::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Self) -> Self {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vec3f {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3f {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}


impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3f {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div for Vec3f {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3f {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl Div<f32> for Vec3f {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vec3f {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

/*--------------------
    Vec4f
--------------------*/ 

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4f {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    pub fn dot(a : Self, b : Self) -> f32 {
        a.x * b.y + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

impl Add for Vec4f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec4f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.y + other.w,
        }
    }
}

impl Sub for Vec4f{
    
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec4f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.y - other.w,
        }
    }
}

impl Mul for Vec4f {
    type Output = Self;

    fn mul(self, other: Self) -> Vec4f {
        Vec4f {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.x * other.z,
            w: self.y * other.w,
        }
    }
}

impl Mul<f32> for Vec4f {
    type Output = Self;

    fn mul(self, other: f32) -> Vec4f {
        Vec4f {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div for Vec4f {
    type Output = Self;

    fn div(self, other: Self) -> Vec4f {
        Vec4f {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.x / other.z,
            w: self.y / other.w,
        }
    }
}

impl Div<f32> for Vec4f {
    type Output = Self;

    fn div(self, other: f32) -> Vec4f {
        Vec4f {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Index<usize> for Vec4f {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => &0.0
        }
    }
}

/*--------------------
    Mat2x2
--------------------*/ 
    
#[derive(Debug, Copy, Clone)]
pub struct Mat2x2f {
    pub values: [Vec2f; 2],
}

impl Mat2x2f {
    pub fn new(
        m00: f32, m01:f32,
        m10: f32, m11:f32) -> Mat2x2f {
        Mat2x2f::from_columns(&[
            Vec2f::new(m00, m01), // COLUMN 1 !!
            Vec2f::new(m10, m11)]) // COLUMN 2 !!
    }

    pub fn from_columns(values: &[Vec2f; 2]) -> Mat2x2f {
        Mat2x2f {
            values: *values
        }
    }

    pub fn transpose(&self) -> Mat2x2f {
        Mat2x2f::new(
            self[0][0], self[1][1],
            self[0][1], self[1][1]
        )
    }

    pub fn rotation(radians: f32) -> Mat2x2f {
        Mat2x2f::new(
            f32::cos(radians), f32::sin(radians),
            -f32::sin(radians), f32::cos(radians)
        )
    }

    pub fn scale(factor: f32) -> Mat2x2f {
        Mat2x2f::new(
            factor, 0.0,
            0.0, factor
        )
    }
}

impl Index<usize> for Mat2x2f {
    type Output = Vec2f;

    fn index(&self, index: usize) -> &Vec2f {
        &self.values[index]
    }
}

// [x,y], x is column, y is row
impl Index<[usize; 2]> for Mat2x2f {
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &f32 {
        &self.values[index[0]][index[1]]
    }
}

impl Mul for Mat2x2f {
    type Output = Mat2x2f;

    fn mul(self, other: Mat2x2f) -> Mat2x2f {
        Mat2x2f::new(
            self[0][0] * other[0][0] + self[1][0] * other[0][1], self[0][0] * other[1][0] + self[1][0] * other[1][1],
            self[0][1] * other[0][0] + self[1][1] * other[0][1], self[0][1] * other[1][0] + self[1][1] * other[1][1]
        )
    }
}

impl Mul<Vec2f> for Mat2x2f {
    type Output = Vec2f;

    fn mul(self, v: Vec2f) -> Vec2f {
        Vec2f::new(
            self[0][0] * v[0] + self[1][0] * v[1],
            self[0][1] * v[0] + self[1][1] * v[1]
        )
    }
}

/*--------------------
    Mat4x4
--------------------*/ 
    
/*
    Notes:

    Boy, writing out coefficient-wise multiply-adds manually is a laborous and errorprone endeavour.
    There's probably a lot of cache trashing with this indexing method and memory layout
    Initializing column-major matrices with columns written out as rows of text is seriously confusing
*/

#[derive(Debug, Copy, Clone)]
pub struct Mat4x4f {
    pub values: [Vec4f; 4],
}

impl Mat4x4f {
    pub fn new(
        m00: f32, m01:f32, m02: f32, m03:f32,
        m10: f32, m11:f32, m12: f32, m13:f32,
        m20: f32, m21:f32, m22: f32, m23:f32,
        m30: f32, m31:f32, m32: f32, m33:f32) -> Mat4x4f {
        Mat4x4f::from_columns(&[
            Vec4f::new(m00, m01, m02, m03), // COLUMN 0 !!
            Vec4f::new(m10, m11, m12, m13), // COLUMN 1 !!
            Vec4f::new(m20, m21, m22, m23), // COLUMN 2 !!
            Vec4f::new(m30, m31, m32, m33) // COLUMN 3 !!
        ])
    }

    pub fn from_columns(values: &[Vec4f; 4]) -> Mat4x4f {
        Mat4x4f {
            values: *values
        }
    }

    pub fn transpose(&self) -> Mat4x4f {
        Mat4x4f::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }

    pub fn rotation_z(radians: f32) -> Mat4x4f {
        Mat4x4f::new(
            f32::cos(radians), f32::sin(radians), 0.0, 0.0,
            -f32::sin(radians), f32::cos(radians), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn scale(factor: f32) -> Mat4x4f {
        Mat4x4f::new(
            factor, 0.0, 0.0, 0.0,
            0.0, factor, 0.0, 0.0,
            0.0, 0.0, factor, 0.0,
            0.0, 0.0, 0.0, factor
        )
    }
}

impl Index<usize> for Mat4x4f {
    type Output = Vec4f;

    fn index(&self, index: usize) -> &Vec4f {
        &self.values[index]
    }
}

// [x,y], x is column, y is row
impl Index<[usize; 2]> for Mat4x4f {
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &f32 {
        &self.values[index[0]][index[1]]
    }
}

impl Mul for Mat4x4f {
    type Output = Mat4x4f;

    fn mul(self, other: Self) -> Self {
        Mat4x4f::new(
            self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2] + self[3][0] * other[0][3],
            self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2] + self[3][0] * other[1][3],
            self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2] + self[3][0] * other[2][3],
            self[0][0] * other[3][0] + self[1][0] * other[3][1] + self[2][0] * other[3][2] + self[3][0] * other[3][3],

            self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2] + self[3][1] * other[0][3],
            self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2] + self[3][1] * other[1][3],
            self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2] + self[3][1] * other[2][3],
            self[0][1] * other[3][0] + self[1][1] * other[3][1] + self[2][1] * other[3][2] + self[3][1] * other[3][3],

            self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2] + self[3][2] * other[0][3],
            self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2] + self[3][2] * other[1][3],
            self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2] + self[3][2] * other[2][3],
            self[0][2] * other[3][0] + self[1][2] * other[3][1] + self[2][2] * other[3][2] + self[3][2] * other[3][3],

            self[0][3] * other[0][0] + self[1][3] * other[0][1] + self[2][3] * other[0][2] + self[3][3] * other[0][3],
            self[0][3] * other[1][0] + self[1][3] * other[1][1] + self[2][3] * other[1][2] + self[3][3] * other[1][3],
            self[0][3] * other[2][0] + self[1][3] * other[2][1] + self[2][3] * other[2][2] + self[3][3] * other[2][3],
            self[0][3] * other[3][0] + self[1][3] * other[3][1] + self[2][3] * other[3][2] + self[3][3] * other[3][3],
        )
    }
}

impl Mul<Vec4f> for Mat4x4f {
    type Output = Vec4f;

    fn mul(self, v: Vec4f) -> Vec4f {
        Vec4f::new(
            self[0][0] * v[0] + self[1][0] * v[1] + self[2][0] * v[2] + self[3][0] * v[3],
            self[0][1] * v[0] + self[1][1] * v[1] + self[2][1] * v[2] + self[3][1] * v[3],
            self[0][2] * v[0] + self[1][2] * v[1] + self[2][2] * v[2] + self[3][2] * v[3],
            self[0][3] * v[0] + self[1][3] * v[1] + self[2][3] * v[2] + self[3][3] * v[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_add_vec3f() {
        let a = Vec3f { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3f { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(a + b, Vec3f {x: 3.0, y: 5.0, z: 7.0 });
    }

    #[test]
    fn test_sub_vec3f() {
        let a = Vec3f { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3f { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(a - b, Vec3f {x: -1.0, y: -1.0, z: -1.0 });
    }

    #[test]
    fn test_dot_vec3f() {
        let a = Vec3f { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3f { x: 2.0, y: 3.0, z: 4.0 };
        assert_approx_eq!(Vec3f::dot(a, b), 20.0);
    }

    #[test]
    fn test_cross_vec3f() {
        let a = Vec3f { x: 1.0, y: 0.0, z: 0.0 };
        let b = Vec3f { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(Vec3f::cross(a,b), Vec3f::new(0.0, 0.0, 1.0)); // Todo: approx_eq
    }

    #[test]
    fn test_mul_mat2x2_vec2f() {
        let m = Mat2x2f::new(
            0.0, 1.0,
            -1.0, 0.0
        );

        let v = Vec2f::new(1.0, 0.0);

        assert_eq!(m * v, Vec2f::new(0.0, 1.0)); // Todo: approx_eq
    }

    #[test]
    fn test_mul_mat4x4_vec4f() {
        let m = Mat4x4f::new(
            0.0, 1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let v = Vec4f::new(1.0, 0.0, 0.0, 0.0);

        assert_eq!(m * v, Vec4f::new(0.0, 1.0, 0.0, 0.0)); // Todo: approx_eq
    }

    #[test]
    fn test_mat4x4_vec3f_translate() {
        let m = Mat4x4f::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            1.0, 2.0, 3.0, 1.0, // translation column values
        );

        let v = Vec4f::new(1.0, 1.0, 1.0, 1.0);

        assert_eq!(m * v, Vec4f::new(2.0, 3.0, 4.0, 1.0)); // Todo: approx_eq
    }
}