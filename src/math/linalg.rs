/*
Some math libs to take inspiration from:

https://www.nalgebra.org/quick_reference/
https://crates.io/crates/cgmath

And there's a bunch of Geometric Algebra libs out there, but then
first we need a basic tour of how to actually use GA. (More reading)

For now, let's create only the types we need.

Todo:
- Write vector arithmetic such that number of dimensions is a generic argument

*/

pub type Vec2f = vec2::Vec2<f32>;
pub type Vec3f = vec3::Vec3<f32>;

mod vec2 {
    use std::ops::Add;
    use std::ops::Sub;
    use std::ops::Mul;

    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    pub struct Vec2<T>
        where T : Add<Output=T> + Sub<Output=T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Vec2<T> 
        where T : Add<Output=T> + Sub<Output=T> + Copy {
        pub fn new(x: T, y: T) -> Vec2<T> {
            Vec2 {
                x: x,
                y: y
            }
        }

        pub fn dot(a : Vec2<T>, b : Vec2<T>) -> T where
            T : Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
            a.x * b.x + a.y * b.y
        }
    }

    impl<T> Add for Vec2<T>
        where T : Add<Output=T> + Sub<Output=T> + Copy {
        
        type Output = Vec2<T>;

        fn add(self, other: Vec2<T>) -> Vec2<T> {
            Vec2 {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl<T> Sub for Vec2<T>
        where T : Add<Output=T> + Sub<Output=T> + Copy {
        
        type Output = Vec2<T>;

        fn sub(self, other: Vec2<T>) -> Vec2<T> {
            Vec2 {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl<T> Mul for Vec2<T>
        where T : Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
        
        type Output = Vec2<T>;

        fn mul(self, other: Vec2<T>) -> Vec2<T> {
            Vec2 {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }
}

mod vec3 {
    use std::ops::Add;
    use std::ops::Sub;
    use std::ops::Mul;

    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    pub struct Vec3<T>
        where T : Add<Output=T> + Sub<Output=T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Vec3<T> 
        where T : Add<Output=T> + Sub<Output=T> + Copy {
        pub fn new(x: T, y: T, z: T) -> Vec3<T> {
            Vec3 {
                x: x,
                y: y,
                z: z,
            }
        }

        // Todo: define using inner product trait?
        pub fn dot(a : Vec3<T>, b : Vec3<T>) -> T where
            T : Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
            a.x * b.x + a.y * b.y + a.z * b.z
        }
    }

    impl<T> Add for Vec3<T>
        where T : Add<Output=T> + Sub<Output=T> + Copy {
        
        type Output = Vec3<T>;

        fn add(self, other: Vec3<T>) -> Vec3<T> {
            Vec3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z
            }
        }
    }

    impl<T> Sub for Vec3<T>
        where T : Add<Output=T> + Sub<Output=T> + Copy {
        
        type Output = Vec3<T>;

        fn sub(self, other: Vec3<T>) -> Vec3<T> {
            Vec3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z
            }
        }
    }

    impl<T> Mul for Vec3<T>
        where T : Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
        
        type Output = Vec3<T>;

        fn mul(self, other: Vec3<T>) -> Vec3<T> {
            Vec3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use assert_approx_eq::assert_approx_eq;

        type Vec3f = Vec3<f32>;

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
            assert_approx_eq!(Vec3::dot(a, b), 20.0);
        }
    }
}

mod matrix {
    use super::Vec3f;

    // Todo: partialEq, Eq and Hash all don't work with f32
    #[derive(Debug, Copy, Clone)]
    pub struct Mat4x4 {
        pub values: [[f32; 4]; 4],
    }

    impl Mat4x4 {
        pub fn new(
            m00: f32, m01: f32, m02: f32, m03: f32,
            m10: f32, m11: f32, m12: f32, m13: f32,
            m20: f32, m21: f32, m22: f32, m23: f32,
            m30: f32, m31: f32, m32: f32, m33: f32) -> Mat4x4 {
            Mat4x4::from_columns(&[
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33]])
        }

        pub const fn from_columns(values: &[[f32; 4]; 4]) -> Mat4x4 {
            Mat4x4 {
                values: *values
            }
        }

        pub fn from_vectors(forward: &Vec3f, up: &Vec3f, right: &Vec3f, position: &Vec3f) -> Mat4x4 {
            Mat4x4::from_columns(&[
                [forward.x, forward.y,  forward.z,  position.x],
                [up.x,      up.y,       up.z,       position.y],
                [right.x,   right.y,    right.z,    position.z],
                [0.0 ,      0.0,        0.0,        1.0]])
        }

        // fn init(self,
        //     m00: f32, m01: f32, m02: f32, m03: f32,
        //     m10: f32, m11: f32, m12: f32, m13: f32,
        //     m20: f32, m21: f32, m22: f32, m23: f32,
        //     m30: f32, m31: f32, m32: f32, m33: f32) -> Mat4x4 {
            
        //     self.values[0][0] = m00; self.values[0][1] = m01; self.values[0][2] = m02; self.values[0][3] = m03;
        //     self.values[1][0] = m10; self.values[1][1] = m11; self.values[1][2] = m12; self.values[1][3] = m13;
        //     self.values[2][0] = m20; self.values[2][1] = m21; self.values[2][2] = m22; self.values[2][3] = m23;
        //     self.values[3][0] = m30; self.values[3][1] = m31; self.values[3][2] = m32; self.values[3][3] = m33;
        // }
    }
}