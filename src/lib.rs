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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }
}

mod vec2 {
    use std::ops::Add;
    use std::ops::Sub;
    use std::ops::Mul;
    use assert_approx_eq::assert_approx_eq;

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

    // Todo: define using inner product trait?
    pub fn dot<T>(a : Vec2<T>, b : Vec2<T>) -> T where
        T : Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
        a.x * b.x + a.y * b.y
    }
}

mod vec3 {
    use std::ops::Add;
    use std::ops::Sub;
    use std::ops::Mul;
    use assert_approx_eq::assert_approx_eq;

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

    // Todo: define using inner product trait?
    pub fn dot<T>(a : Vec3<T>, b : Vec3<T>) -> T where
        T : Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[cfg(test)]
    mod tests {
        use super::*;
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
            assert_approx_eq!(dot(a, b), 20.0);
        }
    }
}
