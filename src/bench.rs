extern crate test;

#[cfg(test)]
mod tests {
    use super::test::{Bencher, black_box};

    use crate::linalg::*;
    use crate::draw::*;
    use crate::resources::*;

    #[bench]
    fn bench_draw_line(b: &mut Bencher) {
        const WIDTH: u32 = 400 * 4;
        const HEIGHT: u32 = 300 * 4;
        let mut screen = Screen::new(WIDTH as usize, HEIGHT as usize);

        let line_color = Color::new(255,255,255);

        b.iter(|| {
            for i in 1..1000 {
                line(&mut screen, Vec2i::new(0,0), Vec2i::new(400,300), &line_color);
                black_box(0);
            }
        });
    }

    #[bench]
    fn bench_draw_triangle_small_screen(b: &mut Bencher) {
        const WIDTH: u32 = 32;
        const HEIGHT: u32 = 32;
        let mut screen = Screen::new(WIDTH as usize, HEIGHT as usize);

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

        let cube = create_cube();
        let verts = cube.verts;
        let tris = cube.tris;
        let uvs = cube.uvs;

        b.iter(|| {
            for j in 1..10 {
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

    #[bench]
    fn bench_draw_triangle_large_screen(b: &mut Bencher) {
        const WIDTH: u32 = 400 * 4;
        const HEIGHT: u32 = 300 * 4;
        let mut screen = Screen::new(WIDTH as usize, HEIGHT as usize);

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

        let cube = create_cube();
        let verts = cube.verts;
        let tris = cube.tris;
        let uvs = cube.uvs;

        b.iter(|| {
            for j in 1..10 {
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