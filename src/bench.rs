extern crate test;

#[cfg(test)]
mod tests {
    use super::test::{Bencher, black_box};

    use crate::linalg::*;
    use crate::draw::*;
    use crate::resources::*;

    // example benchmark
    #[bench]
    fn bench_pow(b: &mut Bencher) {
        // setup
        let x: f64 = 200.0;
        let y: f64 = 301.0;

        b.iter(|| {
            // inner closure performs actual tests
            for i in 1..100 {
                black_box(x.powf(y).powf(x));
            }
        });
    }

    #[bench]
    fn bench_draw_line(b: &mut Bencher) {
        const WIDTH: u32 = 400 * 4;
        const HEIGHT: u32 = 300 * 4;
        const SCREEN_BUFF_SIZE: usize = (WIDTH * HEIGHT * 3) as usize;

        let screen_buffer: Vec<u8> = vec![0; SCREEN_BUFF_SIZE];
        let mut screen = Screen {
            buffer: screen_buffer,
            width: WIDTH as usize,
            height: HEIGHT as usize,
        };

        let line_color = Color::new(255,255,255);

        b.iter(|| {
            for i in 1..1000 {
                line(&mut screen, (0,0), (400,300), &line_color);
                black_box(0);
            }
        });
    }

    #[bench]
    fn bench_draw_triangle(b: &mut Bencher) {
        const WIDTH: u32 = 400 * 4;
        const HEIGHT: u32 = 300 * 4;
        const SCREEN_BUFF_SIZE: usize = (WIDTH * HEIGHT * 3) as usize;

        let screen_buffer: Vec<u8> = vec![0; SCREEN_BUFF_SIZE];
        let mut screen = Screen {
            buffer: screen_buffer,
            width: WIDTH as usize,
            height: HEIGHT as usize,
        };

        // vert buffer
        let verts = vec!(
            Vec4f::new(-1.0, -1.0, -1.0, 1.0),
            Vec4f::new(-1.0,  1.0, -1.0, 1.0),
            Vec4f::new( 1.0,  1.0, -1.0, 1.0),
            Vec4f::new( 1.0, -1.0, -1.0, 1.0),
            Vec4f::new(-1.0, -1.0,  1.0, 1.0),
            Vec4f::new(-1.0,  1.0,  1.0, 1.0),
            Vec4f::new( 1.0,  1.0,  1.0, 1.0),
            Vec4f::new( 1.0, -1.0,  1.0, 1.0));

        // index buffer
        let tris = vec!(
            // front
            0, 1, 2, 
            0, 2, 3,

            // back
            6, 5, 4, 
            7, 6, 4,

            // left
            4, 5, 1, 
            4, 1, 0,

            // right
            3, 2, 6, 
            3, 6, 7,
            
            // top
            1, 5, 6, 
            1, 6, 2,

            // bottom
            7, 4, 0, 
            3, 7, 0);

        let uvs = vec!(
            // front
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
            Vec2f::new(0.0, 0.0), Vec2f::new(1.0, 1.0), Vec2f::new(1.0, 0.0),

            // back
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), 
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0),

            // left
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), 
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0),

            // right
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), 
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0),
            
            // top
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), 
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0),

            // bottom
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), 
            Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0),
        );
        
        let cam_mat = Mat4x4f::translation(0.0, 0.0, -8.0);
        let cam_mat_inverse = cam_mat.inverse();

        // Camera projection matrix
        let near: f32 = 0.1;
        let far: f32 = 1000.0;
        let fov: f32 = 80.0;
        let aspect: f32 =  HEIGHT as f32 / WIDTH as f32;
        let proj_mat = Mat4x4f::projection(near, far, aspect, fov);

        let tri_mat = Mat4x4f::identity();
        
        let tex = load_texture(String::from("resources/test.png")).unwrap();

        b.iter(|| {
            for i in 1..10 {
                let num_tris = tris.len() / 3;
                for i in 0..num_tris {
                    triangle(
                        &verts[tris[i*3 + 0]],
                        &verts[tris[i*3 + 1]],
                        &verts[tris[i*3 + 2]],
                        &uvs[i*3 + 0],
                        &uvs[i*3 + 1],
                        &uvs[i*3 + 2],
                        &tex,
                        &tri_mat,
                        &cam_mat_inverse,
                        &proj_mat,
                        &mut screen);
                }
                black_box(0);
            }
        });
    }
}