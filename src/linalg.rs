/*
Some math libs to take inspiration from:

https://crates.io/crates/cgmath
https://github.com/rustsim (NAlgebra, NPhysics, NCollide, Alga)
https://crates.io/crates/num

And there's a bunch of Geometric Algebra libs out there, but then
first we need a basic tour of how to actually use GA. (More reading)

For now, let's create only the types we need.

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
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_add() {
        let a = Vec3f { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3f { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(a + b, Vec3f {x: 3.0, y: 5.0, z: 7.0 });
    }

    #[test]
    fn test_sub() {
        let a = Vec3f { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3f { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(a - b, Vec3f {x: -1.0, y: -1.0, z: -1.0 });
    }

    #[test]
    fn test_dot() {
        let a = Vec3f { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3f { x: 2.0, y: 3.0, z: 4.0 };
        assert_approx_eq!(Vec3f::dot(a, b), 20.0);
    }

    #[test]
    fn test_mat2x2_mul() {
        let m = Mat2x2f::new(
            0.0, 1.0,
            -1.0, 0.0
        );

        let v = Vec2f::new(1.0, 0.0);

        assert_eq!(m * v, Vec2f::new(0.0, 1.0));
    }
}