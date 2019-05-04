/*
A simple linear algebra library, far from finished.

Uses left-handed coordinate system.


Some math libs to take inspiration from:

https://crates.io/crates/cgmath
https://github.com/rustsim (NAlgebra, NPhysics, NCollide, Alga)
https://crates.io/crates/num
https://github.com/JeroenDStout/Ap-Code-BlackRoot

And there's a bunch of Geometric Algebra libs out there, but then
first we need a basic tour of how to actually use GA. (More reading)

For now, let's create only the types we need.

Todo:
- Impl approx_eq for testing https://docs.rs/approx/0.3.2/approx/
- Impl traits for references to types, such that caller doesn't need to explicitly dereference

Todo much later:
- Optimize matrix operations by handling affine and projective 4x4 multiplies separately
- Specification of generic Scalar type trait bound
- Write vector arithmetic such that number of dimensions is a generic argument
- partialEq, Eq and Hash all don't work with f32, but work fine for fixed point types

*/

#![allow(dead_code)]

extern crate float_cmp;

use std::ops::*;
use float_cmp::{Ulps, ApproxEq};

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

// Todo: could return a ref to same memory, really
// Another example where reinterpret cast to const& could be good for perf
impl From<&Vec4f> for Vec2f {
    fn from(item: &Vec4f) -> Self {
        Vec2f::new(item.x, item.y)
    }
}

impl From<&Vec3f> for Vec2f {
    fn from(item: &Vec3f) -> Self {
        Vec2f::new(item.x, item.y)
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

    pub fn length(&self) -> f32 {
        f32::sqrt(Vec3f::dot(self, self))
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        Vec3f::new(
            self.x / len,
            self.y / len,
            self.z / len
        )
    }

    // Todo: define using inner product trait?
    pub fn dot(a : &Vec3f, b : &Vec3f) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a : &Vec3f, b : &Vec3f) -> Self {
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

// Todo: will I always need to implemented for copy AND ref separately?
impl Mul<&f32> for Vec3f {
    type Output = Self;

    fn mul(self, other: &f32) -> Self {
        Vec3f {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl MulAssign<&f32> for Vec3f {
    fn mul_assign(&mut self, other: &f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
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

// Todo: can we do reinterpret cast? We're now cloning...
// Feels like you'd slide into unsafe transmute_cast territory
impl From<&Vec4f> for Vec3f {
    fn from(item: &Vec4f) -> Self {
        Vec3f::new(item.x, item.y, item.z)
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

    pub fn norm_by_w(a: &Self) -> Self {
        Vec4f {
            x: a.x / a.w,
            y: a.y / a.w,
            z: a.z / a.w,
            w: a.w
        }
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
            _ => panic!("Trying to index non-existing coefficient on Vec4f: {}", index)
        }
    }
}

impl IndexMut<usize> for Vec4f {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Trying to index non-existing coefficient on Vec4f: {}", index)
        }
    }
}

impl ApproxEq for Vec4f {
    type Flt = f32;

    fn approx_eq(&self, other: &Self, epsilon: <f32 as ApproxEq>::Flt, ulps: <<f32 as ApproxEq>::Flt as Ulps>::U) -> bool {
        self.x.approx_eq(&other.x, epsilon, ulps) &&
        self.y.approx_eq(&other.y, epsilon, ulps) &&
        self.z.approx_eq(&other.z, epsilon, ulps) &&
        self.w.approx_eq(&other.w, epsilon, ulps)
    }
}

/*--------------------
    Mat2x2
--------------------*/ 
    
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat2x2f {
    pub values: [Vec2f; 2],
}

// Todo! Switch initializer arguments to the more legible, non-transposed order
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
*/

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4x4f {
    pub values: [Vec4f; 4],
}

impl Mat4x4f {
    pub fn zero() -> Mat4x4f {
        Mat4x4f::from_columns(&[
            Vec4f::new(0.0,0.0,0.0,0.0),
            Vec4f::new(0.0,0.0,0.0,0.0),
            Vec4f::new(0.0,0.0,0.0,0.0),
            Vec4f::new(0.0,0.0,0.0,0.0)
        ])
    }

    pub fn identity() -> Mat4x4f {
        Mat4x4f::from_columns(&[
            Vec4f::new(1.0,0.0,0.0,0.0),
            Vec4f::new(0.0,1.0,0.0,0.0),
            Vec4f::new(0.0,0.0,1.0,0.0),
            Vec4f::new(0.0,0.0,0.0,1.0)
        ])
    }

    pub fn new(
        m00: f32, m10:f32, m20: f32, m30:f32,
        m01: f32, m11:f32, m21: f32, m31:f32,
        m02: f32, m12:f32, m22: f32, m32:f32,
        m03: f32, m13:f32, m23: f32, m33:f32) -> Mat4x4f {
        Mat4x4f::from_columns(&[
            Vec4f::new(m00, m01, m02, m03),
            Vec4f::new(m10, m11, m12, m13),
            Vec4f::new(m20, m21, m22, m23),
            Vec4f::new(m30, m31, m32, m33)
        ])
    }

    pub fn from_columns(values: &[Vec4f; 4]) -> Mat4x4f {
        Mat4x4f {
            values: *values
        }
    }

    pub fn transpose(&self) -> Mat4x4f {
        let mut m = Mat4x4f::zero();

        for x in 0..4 {
            for y in 0..4 {
                m[[x,y]] = self[[y,x]];
            }
        }

        m

        // Literal:
        // Mat4x4f::new(
        //     self[0][0], self[1][0], self[2][0], self[3][0],
        //     self[0][1], self[1][1], self[2][1], self[3][1],
        //     self[0][2], self[1][2], self[2][2], self[3][2],
        //     self[0][3], self[1][3], self[2][3], self[3][3],
        // )
    }

    // As per Lengyel's book Foundations of Game Engine Development
    // Inverts any 4x4 using optimal amount of operations
    // Todo: since we'll be using Transform matrices a lot,
    // have a specialized function for inverting using 3x3 submatrix.
    pub fn inverse(&self) -> Mat4x4f {
        let a: Vec3f = (&self[0]).into();
        let b: Vec3f = (&self[1]).into();
        let c: Vec3f = (&self[2]).into();
        let d: Vec3f = (&self[3]).into();

        let x = &self[[0,3]];
        let y = &self[[1,3]];
        let z = &self[[2,3]];
        let w = &self[[3,3]];

        let mut s = Vec3f::cross(&a, &b);
        let mut t = Vec3f::cross(&c, &d);
        let mut u = a * y - b * x;
        let mut v = c * w - d * z;

        // Todo: needs a det(m) == 0 check, otherwise we get inf->nan
        let inv_det = 1.0 / (Vec3f::dot(&s, &v) + Vec3f::dot(&t, &u));

        // println!("det: {:?}", (Vec3f::dot(s, v) + Vec3f::dot(t, u)));

        s *= inv_det;
        t *= inv_det;
        u *= inv_det;
        v *= inv_det;

        let r0 = Vec3f::cross(&b, &v) + t * y;
        let r1 = Vec3f::cross(&v, &a) - t * x;
        let r2 = Vec3f::cross(&d, &u) + s * w;
        let r3 = Vec3f::cross(&u, &c) - s * z;

        Mat4x4f::new(
            r0.x, r0.y, r0.z, -Vec3f::dot(&b, &t),
            r1.x, r1.y, r1.z,  Vec3f::dot(&a, &t),
            r2.x, r2.y, r2.z, -Vec3f::dot(&d, &s),
            r3.x, r3.y, r3.z,  Vec3f::dot(&c, &s),
        )
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Mat4x4f {
        Mat4x4f::new(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn rotation_x(radians: f32) -> Mat4x4f {
        Mat4x4f::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, f32::cos(radians), -f32::sin(radians), 0.0,
            0.0, f32::sin(radians), f32::cos(radians), 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn rotation_y(radians: f32) -> Mat4x4f {
        Mat4x4f::new(
            f32::cos(radians), 0.0, -f32::sin(radians), 0.0,
            0.0, 1.0, 0.0, 0.0,
            f32::sin(radians), 0.0, f32::cos(radians), 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn rotation_z(radians: f32) -> Mat4x4f {
        Mat4x4f::new(
            f32::cos(radians), -f32::sin(radians), 0.0, 0.0,
            f32::sin(radians), f32::cos(radians), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn scale_uniform(factor: f32) -> Mat4x4f {
        Mat4x4f::new(
            factor, 0.0, 0.0, 0.0,
            0.0, factor, 0.0, 0.0,
            0.0, 0.0, factor, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Mat4x4f {
        Mat4x4f::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn projection(near: f32, far: f32, aspect: f32, fov: f32) -> Mat4x4f {
        let fov_rad: f32 = 1.0 / f32::tan(fov * 0.5 / 180.0 * std::f32::consts::PI);

        let mut proj_mat = Mat4x4f::identity();
        proj_mat[0][0] = aspect * fov_rad;
        proj_mat[1][1] = fov_rad;
        proj_mat[2][2] = far / (far - near);
        proj_mat[2][3] = 1.0;
        proj_mat[3][2] = far * near / (far - near);
        proj_mat[3][3] = 0.0;

        proj_mat
    }
}

impl Index<usize> for Mat4x4f {
    type Output = Vec4f;

    fn index(&self, index: usize) -> &Vec4f {
        &self.values[index]
    }
}

impl IndexMut<usize> for Mat4x4f {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Vec4f {
        &mut self.values[index]
    }
}

// NOTE: [x,y], x is column, y is row. Lengyel has this the other way around.
impl Index<[usize; 2]> for Mat4x4f {
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &f32 {
        &self.values[index[0]][index[1]]
    }
}

impl IndexMut<[usize; 2]> for Mat4x4f {
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f32 {
        &mut self.values[index[0]][index[1]]
    }
}

impl Mul for Mat4x4f {
    type Output = Mat4x4f;

    // Todo: use a macro to generate this, obviously
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

    // Todo: use a macro to generate this, obviously
    fn mul(self, v: Vec4f) -> Vec4f {
        Vec4f::new(
            self[0][0] * v[0] + self[1][0] * v[1] + self[2][0] * v[2] + self[3][0] * v[3],
            self[0][1] * v[0] + self[1][1] * v[1] + self[2][1] * v[2] + self[3][1] * v[3],
            self[0][2] * v[0] + self[1][2] * v[1] + self[2][2] * v[2] + self[3][2] * v[3],
            self[0][3] * v[0] + self[1][3] * v[1] + self[2][3] * v[2] + self[3][3] * v[3]
        )
    }
}

impl ApproxEq for Mat4x4f {
    type Flt = f32;

    fn approx_eq(&self, other: &Self, epsilon: <f32 as ApproxEq>::Flt, ulps: <<f32 as ApproxEq>::Flt as Ulps>::U) -> bool {
        self[0].approx_eq(&other[0], epsilon, ulps) &&
        self[1].approx_eq(&other[1], epsilon, ulps) &&
        self[2].approx_eq(&other[2], epsilon, ulps) &&
        self[3].approx_eq(&other[3], epsilon, ulps)
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
        assert_approx_eq!(Vec3f::dot(&a, &b), 20.0);
    }

    #[test]
    fn test_cross_vec3f() {
        let a = Vec3f { x: 1.0, y: 0.0, z: 0.0 };
        let b = Vec3f { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(Vec3f::cross(&a,&b), Vec3f::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_mul_mat2x2_vec2f() {
        let m = Mat2x2f::new(
            0.0, 1.0,
            -1.0, 0.0
        );

        let v = Vec2f::new(1.0, 0.0);

        assert_eq!(m * v, Vec2f::new(0.0, 1.0));
    }

    #[test]
    fn test_mul_mat4x4_vec4f() {
        let m = Mat4x4f::new(
            0.0, -1.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let v = Vec4f::new(1.0, 0.0, 0.0, 0.0);

        assert_eq!(m * v, Vec4f::new(0.0, 1.0, 0.0, 0.0));
    }

    #[test]
    fn test_inverse() {
        let m_scale = Mat4x4f::scale(2.0, 3.0, 4.0);
        let m_rot = Mat4x4f::rotation_z(std::f32::consts::PI);
        let m_trs = Mat4x4f::translation(0.0, 0.0, 1.0);
        let m = m_rot * m_scale * m_trs;
        println!("m : {:?}", m);
        let m_inv = m.inverse();
        println!("m': {:?}", m_inv);

        assert!((m_inv * m).approx_eq(&Mat4x4f::identity(), 2.0 * ::std::f32::EPSILON, 2));
    }

    #[test]
    fn test_mat4x4_vec3f_translate() {
        let m = Mat4x4f::new(
            1.0, 0.0, 0.0, 1.0,
            0.0, 2.0, 0.0, 2.0, // scale y by 2
            0.0, 0.0, 1.0, 3.0,
            0.0, 0.0, 0.0, 1.0, // translate by [1,2,3]
        );

        let v = Vec4f::new(1.0, 1.0, 1.0, 1.0);

        assert_eq!(m * v, Vec4f::new(2.0, 4.0, 4.0, 1.0));
    }

    #[test]
    fn test_indexers() {
        let mut x = Vec4f::new(0.0,0.0,0.0,1.0);
        x[1] = 1.0;

        let mut m = Mat4x4f::zero();

        m[0] = Vec4f::new(0.0,0.0,0.0,1.0);
        m[[0, 1]] = 0.0;
    }
}