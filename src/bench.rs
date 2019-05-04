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
        let cam_inv = cam_mat.inverse();

        let near: f32 = 0.1;
        let far: f32 = 1000.0;
        let fov: f32 = 80.0;
        let aspect: f32 =  HEIGHT as f32 / WIDTH as f32;
        let cam_proj = Mat4x4f::projection(near, far, aspect, fov);

        let obj_mat = Mat4x4f::identity();
        
        let tex = load_texture(String::from("resources/test.png")).unwrap();

        let mesh = create_cube();

        b.iter(|| {
            for _j in 1..10 {
                draw_mesh(&mesh, &tex, &obj_mat, &cam_inv, &cam_proj, &mut screen);
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
        let cam_inv = cam_mat.inverse();

        let near: f32 = 0.1;
        let far: f32 = 1000.0;
        let fov: f32 = 80.0;
        let aspect: f32 =  HEIGHT as f32 / WIDTH as f32;
        let cam_proj = Mat4x4f::projection(near, far, aspect, fov);

        let obj_mat = Mat4x4f::identity();
        
        let tex = load_texture(String::from("resources/test.png")).unwrap();

        let mesh = create_cube();

        b.iter(|| {
            for _j in 1..10 {
                draw_mesh(&mesh, &tex, &obj_mat, &cam_inv, &cam_proj, &mut screen);
                black_box(0);
            }
        });
    }
}