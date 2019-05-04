#![feature(test)] // for benchmark feature

extern crate sdl2;
extern crate gl;

use sdl2::rect::{Rect};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator};
use std::{thread, time};

pub mod draw;
pub mod linalg;
pub mod resources;
mod bench; // Not exactly sure why, but I need this otherwise my benches don't run

use linalg::*;
use draw::*;
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
    const WIDTH: u32 = 400 * 2;
    const HEIGHT: u32 = 300 * 2;

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
    let mut screen = Screen::new(WIDTH as usize, HEIGHT as usize);

    // Load our cube mesh
    let mesh = create_cube();

    // Camera projection matrix
    let near: f32 = 0.1;
    let far: f32 = 1000.0;
    let fov: f32 = 80.0;
    let aspect: f32 =  HEIGHT as f32 / WIDTH as f32;
    let cam_proj = Mat4x4f::projection(near, far, aspect, fov);

    // Clear screen before doing anything
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Our test texture
    let tex = load_texture(String::from("resources/checker.png")).unwrap();

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

        // Rendering

        // Clear our buffer
        draw::clear_color(&mut screen);
        draw::clear_depth(&mut screen);

        // Cam setup
        let cam = Mat4x4f::translation(0.0, 0.0, -8.0);
        let cam_inv = cam.inverse();

        // Let's draw our cube

        // rotate and translate it in world space
        let obj1_mat = 
            Mat4x4f::translation(0.0, f32::sin(time * 1.0) * 1.0, 0.0) *
            Mat4x4f::rotation_y(f32::sin(time * 3.0) * 1.0) *
            Mat4x4f::rotation_x(f32::sin(time * 1.333) * 1.0);

        let obj2_mat = 
            Mat4x4f::translation(f32::sin(time * 1.3221) * 2.0, 0.0, 0.0) *
            Mat4x4f::rotation_y(f32::sin(time * 2.0) * 1.0) *
            Mat4x4f::rotation_x(f32::sin(time * 1.7672) * 1.0);

        let obj3_mat = 
            Mat4x4f::scale_uniform(0.5) *
            Mat4x4f::translation(f32::cos(time * 0.5) * 3.0, f32::sin(time * 1.3221) * 3.0, f32::sin(time * 1.3221) * 3.0) *
            Mat4x4f::rotation_y(f32::cos(time * 3.1) * 1.0) *
            Mat4x4f::rotation_x(f32::sin(time * -1.0672) * 1.0);
        // let tri_mat = Mat4x4f::identity();
        
        draw_mesh(&mesh, &tex, &obj1_mat, &cam_inv, &cam_proj, &mut screen);
        draw_mesh(&mesh, &tex, &obj2_mat, &cam_inv, &cam_proj, &mut screen);
        draw_mesh(&mesh, &tex, &obj3_mat, &cam_inv, &cam_proj, &mut screen);

        // Copy screenbuffer to texture
        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            buffer.copy_from_slice(screen.color.as_ref());
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

fn draw_mesh(mesh: &Mesh, tex: &Vec<Color>, transform: &Mat4x4f, cam_inv: &Mat4x4f, cam_proj: &Mat4x4f, screen: &mut Screen) {
    let verts = &mesh.verts;
    let tris = &mesh.tris;
    let uvs = &mesh.uvs;

    let num_tris = tris.len() / 3;
        for i in 0..num_tris {
            draw::triangle(
                &verts[tris[i*3 + 0]],
                &verts[tris[i*3 + 1]],
                &verts[tris[i*3 + 2]],
                &uvs[i*3 + 0],
                &uvs[i*3 + 1],
                &uvs[i*3 + 2],
                tex,
                transform,
                cam_inv,
                cam_proj,
                screen);
        }
}