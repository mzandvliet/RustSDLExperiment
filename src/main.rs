#![feature(test)]

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
mod resources;
mod bench;

use linalg::*;
use resources::*;

/*
    Use From/Into trait impls to get around all the explicit casting
    Pass by immutable reference more, instead of by copy. This is not C#, mister.

    Then:
    Implement line clipping
    Implement frustum culling
    Implement triangle clipping
    Implement more of Lengyel's book
*/


fn main() {
    do_game().unwrap();
}

fn do_game() -> Result<(), String> {
    const WIDTH: u32 = 400 * 4;
    const HEIGHT: u32 = 300 * 4;
    const SCREEN_BUFF_SIZE: usize = (WIDTH * HEIGHT * 3) as usize;

    // Initialize SDL

    let sdl_context = sdl2::init()?;
    let mut event_pump = sdl_context.event_pump()?;
    let video_subsystem = sdl_context.video()?;

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

    // Texture used to blit our screen buffer to SDL canvas
    let texture_creator : TextureCreator<_> = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT).map_err(|e| e.to_string())?;

    // Create our cpu-side screen buffer
    let screen_buffer: Vec<u8> = vec![0; SCREEN_BUFF_SIZE];
    let mut screen = draw::Screen {
        buffer: screen_buffer,
        width: WIDTH as usize,
        height: HEIGHT as usize,
    };
    
    // Load our cube mesh
    let mesh = create_cube();
    let verts = mesh.verts;
    let tris = mesh.tris;
    let uvs = mesh.uvs;

    // let mesh = create_test_triangle();
    // let verts = mesh.verts;
    // let tris = mesh.tris;
    // let uvs = mesh.uvs;

    // Camera projection matrix
    let near: f32 = 0.1;
    let far: f32 = 1000.0;
    let fov: f32 = 80.0;
    let aspect: f32 =  HEIGHT as f32 / WIDTH as f32;
    let proj_mat = Mat4x4f::projection(near, far, aspect, fov);

    // Clear screen before doing anything
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Our test texture
    let tex = load_texture(String::from("resources/test.png")).unwrap();

    let mut frame : u32 = 0;
    let mut time = 0.0;

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
        let tri_mat = 
            Mat4x4f::translation(0.0, f32::sin(time * 1.0) * 1.0, 0.0) *
            Mat4x4f::rotation_y(time * 1.0) *
            Mat4x4f::rotation_x(f32::sin(time * 2.0) * 0.5);
        // let tri_mat = Mat4x4f::identity();
        
        // draw all tris in sequence
        let num_tris = tris.len() / 3;
        let i = 1;
        // for i in 0..num_tris {
            draw::triangle(
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
        // }

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

/*
    Todo: the below are unused as of now. Still need to clip lines
    and triangles to the screen bounds...
*/

// fn is_line_visible(a: (i32, i32), b: (i32, i32), screen_dims: (i32, i32)) -> bool {
//     is_point_visible(a, screen_dims) || is_point_visible(b, screen_dims)
// }

// fn is_point_visible(p: (i32, i32), screen_dims: (i32, i32)) -> bool {
//     p.0 >= 0 && p.0 < screen_dims.0 &&
//     p.1 >= 0 && p.1 < screen_dims.1
// }

// fn clip_line(a: (i32, i32), b: (i32, i32), s: (i32, i32)) -> ((i32,i32),(i32,i32)) {
//     // let bot_intersect = intersect_line((a, b), 0, 0);
//     // if bot_intersect.0 >= 0 && bot_intersect.1 < s.1 {
//         // Wait, now I still don't know whether a, b, or both points should be clipped
//         // Maybe I don't need to know...
//     // }

//     (a, b)
// }

// fn intersect_line(a: ((i32,i32),(i32,i32)), slope: i32, inter: i32) -> (i32, i32) {
//     let a_rr = slope_intercept(a);

//     let x = intersect(a_rr.0, a_rr.1, slope, inter);
//     (x, a_rr.0 * x + a_rr.1)
// }

// fn intersect_lines(a: ((i32,i32),(i32,i32)), b: ((i32,i32),(i32,i32))) -> (i32, i32) {
//     let a_rr = slope_intercept(a);
//     let b_rr = slope_intercept(b);

//     let x = intersect(a_rr.0, a_rr.1, b_rr.0, b_rr.1);
//     (x, a_rr.0 * x + a_rr.1)
// }

// fn slope_intercept(a: ((i32,i32),(i32,i32))) -> (i32, i32) {
//     let slope = ((a.0).1 - (a.1).1) / ((a.0).0 - (a.1).0);
//     let inter = (a.0).1 - slope * (a.0).0;
//     (slope, inter)
// }

// fn intersect(a: i32, b: i32, c: i32, d: i32) -> i32 {
//     (d - b) / (a - c) // Todo: precision, man. Rounding.
// }

