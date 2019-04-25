
extern crate sdl2;
extern crate gl;

use sdl2::rect::{Rect};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator};
use std::{thread, time};

mod draw;
mod linalg;

use linalg::*;

/*
    Use From/Into trait impls to get around all the explicit casting
    Pass by immutable reference more, instead of by copy. This is not C#, mister.

    Then:
    Implement back-face culling (normals, or winding order)
    Implement line clipping
    Implement frustum culling
    Implement triangle clipping
    Implement a solid, shaded, triangle rasterizer
    Implement more of Lengyel's book
*/


fn main() {
    do_game().unwrap();
}

fn do_game() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 300;
    const SCREEN_BUFF_SIZE: usize = (WIDTH * HEIGHT * 3) as usize;

    let window = video_subsystem
        .window("Spinning Cube", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    // Clear screen before doing anything
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Our cpu-side screen buffer
    let screen_buffer: Box<[u8]> = Box::new([0; SCREEN_BUFF_SIZE]);
    let mut screen = draw::Screen {
        buffer: screen_buffer,
        width: WIDTH as usize,
        height: HEIGHT as usize,
    };
    
    // Texture used to blit our screen buffer to SDL canvas
    let texture_creator : TextureCreator<_> = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame : u32 = 0;
    let mut time = 0.0;

    // Cube mesh

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

    // Project matrix
    let near: f32 = 0.1;
    let far: f32 = 1000.0;
    let fov: f32 = 80.0;
    let aspect: f32 =  HEIGHT as f32 / WIDTH as f32;
    let proj_mat = Mat4x4f::projection(near, far, aspect, fov);

    'running: loop {
        // Game simulation logic
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    //do something interesting
                }
                _ => {}
            }
        }

        // Do some game logic

        // Rendering

        // Clear our buffer
        draw::clear(&mut screen);

        // Cam setup
        let cam_mat = Mat4x4f::translation(0.0, 0.0, -8.0);
        let cam_mat_inverse = cam_mat.inverse();

        // Let's draw our cube

        // rotate and translate it in world space
        let tri_mat = Mat4x4f::translation(0.0, f32::sin(time * 1.0) * 2.0, 0.0) * Mat4x4f::rotation_y(time * 2.0);
        
        // draw all tris in sequence
        let num_tris = tris.len() / 3;
        for i in 0..num_tris { // 0..num_tris
            draw_triangle(
                &verts[tris[i*3 + 0]],
                &verts[tris[i*3 + 1]],
                &verts[tris[i*3 + 2]],
                &tri_mat,
                &cam_mat_inverse,
                &proj_mat,
                &mut screen);
        }

        // Copy screenbuffer to texture
        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            buffer.copy_from_slice(screen.buffer.as_ref());
        })?;

        // Blit it to canvas
        let screen_rect = Rect::new(0,0,WIDTH,HEIGHT);
        canvas.copy(&texture, screen_rect, screen_rect)?;

        canvas.present();

        frame += 1;

        // Todo: measure time passed since last frame, use to finetune sleep timing
        //let now = time::Instant::now;
        let delta_time = time::Duration::from_millis(16);
        time += 0.016;
        thread::sleep(delta_time);
    }

    Ok(())
}

fn draw_triangle(p1: &Vec4f, p2: &Vec4f, p3: &Vec4f, obj_mat: &Mat4x4f, cam_inv: &Mat4x4f, cam_proj: &Mat4x4f, screen: &mut draw::Screen) {
    // Todo: 
    // - split this into multiple stages, of course, and
    // - loop over a list of points instead
    // - do backface culling before rendering solids

    // Obj to world
    let p1 = *obj_mat * *p1;
    let p2 = *obj_mat * *p2;
    let p3 = *obj_mat * *p3;

    let normal = Vec3f::cross(&(&(p2 - p1)).into(), &(&(p3 - p1)).into()); // todo: lol, fix dis ref/deref mess
    let normal = normal.normalize();

    let cam_to_tri: Vec3f = Vec3f::from(&p1) - Vec3f::new(0.0, 0.0, -8.0);
    if Vec3f::dot(&cam_to_tri, &normal) < 0.0 {
        // Lighting
        let light_dir = Vec3f::new(0.0, -0.5, 1.0).normalize();
        let l_dot_n = f32::max(0.0, -Vec3f::dot(&normal, &light_dir));

        // World to camera space
        let p1 = *cam_inv * p1;
        let p2 = *cam_inv * p2;
        let p3 = *cam_inv * p3;

        // Projection
        let p1 = cam_proj.mul_norm(&p1);
        let p2 = cam_proj.mul_norm(&p2);
        let p3 = cam_proj.mul_norm(&p3);

        let p1 = Vec2f::from(&p1);
        let p2 = Vec2f::from(&p2);
        let p3 = Vec2f::from(&p3);

        // println!("{:?}, {:?}, {:?}", p1s, p2s, p3s);

        let shaded_color = draw::Color::new((255.0 * l_dot_n) as u8, (100.0 * l_dot_n) as u8, (150.0 * l_dot_n) as u8);
        let wire_color = draw::Color::new(255, 255, 255);
        draw::triangle_solid(screen, p1, p2, p3, &shaded_color);
        draw::triangle_wireframe(screen, p1, p2, p3, &wire_color);
    }
}

fn is_line_visible(a: (i32, i32), b: (i32, i32), screen_dims: (i32, i32)) -> bool {
    is_point_visible(a, screen_dims) || is_point_visible(b, screen_dims)
}

fn is_point_visible(p: (i32, i32), screen_dims: (i32, i32)) -> bool {
    p.0 >= 0 && p.0 < screen_dims.0 &&
    p.1 >= 0 && p.1 < screen_dims.1
}

fn clip_line(a: (i32, i32), b: (i32, i32), s: (i32, i32)) -> ((i32,i32),(i32,i32)) {
    // let bot_intersect = intersect_line((a, b), 0, 0);
    // if bot_intersect.0 >= 0 && bot_intersect.1 < s.1 {
        // Wait, now I still don't know whether a, b, or both points should be clipped
        // Maybe I don't need to know...
    // }

    (a, b)
}

fn intersect_line(a: ((i32,i32),(i32,i32)), slope: i32, inter: i32) -> (i32, i32) {
    let a_rr = slope_intercept(a);

    let x = intersect(a_rr.0, a_rr.1, slope, inter);
    (x, a_rr.0 * x + a_rr.1)
}

fn intersect_lines(a: ((i32,i32),(i32,i32)), b: ((i32,i32),(i32,i32))) -> (i32, i32) {
    let a_rr = slope_intercept(a);
    let b_rr = slope_intercept(b);

    let x = intersect(a_rr.0, a_rr.1, b_rr.0, b_rr.1);
    (x, a_rr.0 * x + a_rr.1)
}

fn slope_intercept(a: ((i32,i32),(i32,i32))) -> (i32, i32) {
    let slope = ((a.0).1 - (a.1).1) / ((a.0).0 - (a.1).0);
    let inter = (a.0).1 - slope * (a.0).0;
    (slope, inter)
}

fn intersect(a: i32, b: i32, c: i32, d: i32) -> i32 {
    (d - b) / (a - c) // Todo: precision, man. Rounding.
}

fn perspective_divide(point: Vec4f) -> Vec4f {
    Vec4f::new(point.x / point.z, point.y / point.z, point.z, 1.0)
}

// Todo: don't hardcode the resolution
fn to_screenspace(point: Vec4f) -> Vec4f {
    let w = 1.33;
    let h = 1.0;
    Vec4f::new((point.x + 0.5 * w) / w, point.y + 0.5 * h, point.z, 1.0)
}

