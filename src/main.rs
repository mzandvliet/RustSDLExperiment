
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
    Prototype:
    Define a cube using 3d points
    Project to 2d screen using a matrix
    Draw using bresenham

    Then:
    Implement  Lengyel's book, plus a rasterizer

    use From/Into to get around all the explicit casting
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
        .window("rust-sdl-demo", WIDTH, HEIGHT)
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

    // Our screen buffer
    let screen_buffer: Box<[u8]> = Box::new([0; SCREEN_BUFF_SIZE]);
    let mut screen = draw::Screen {
        buffer: screen_buffer,
        width: WIDTH as usize,
        height: HEIGHT as usize,
    };
    
    // Texture used to blit our screen buffer to canvas
    let texture_creator : TextureCreator<_> = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
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
                    //do something
                }
                _ => {}
            }
        }

        if frame >= 30 {
            //game.update();
            frame = 0;
        }

        // Rendering

        // Clear
        draw::clear(&mut screen);
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        //let cam_mat = Mat4x4f::translation(0.0, 0.0, -1.0); // f32::sin(3.1423 * time) * 2.0
        let cam_mat = Mat4x4f::translation(0.0, 0.0, -6.0 + f32::sin(3.1423 * time) * -3.0) * Mat4x4f::rotation_z(time * 0.5);
        let cam_mat_inverse = cam_mat.inverse();

        let p1 = Vec4f::new(-1.0, -0.5, 2.0, 1.0);
        let p2 = Vec4f::new(0.0, 1.0, 2.0, 1.0);
        let p3 = Vec4f::new(1.0, -0.5, 2.0, 1.0);

        let p1 = cam_mat_inverse * p1;
        let p2 = cam_mat_inverse * p2;
        let p3 = cam_mat_inverse * p3;

        let p1 = perspective(p1);
        let p2 = perspective(p2);
        let p3 = perspective(p3);

        let p1 = screenspace(p1);
        let p2 = screenspace(p2);
        let p3 = screenspace(p3);

        let p1 = pixelspace(p1);
        let p2 = pixelspace(p2);
        let p3 = pixelspace(p3);

        let screen_dims = (screen.width as i32, screen.height as i32);

        let p1s = clip((p1.x as i32, p1.y as i32), screen_dims);
        let p2s = clip((p2.x as i32, p2.y as i32), screen_dims);
        let p3s = clip((p3.x as i32, p3.y as i32), screen_dims);

        // println!("{:?}, {:?}, {:?}", p1s, p2s, p3s);

        draw::triangle(&mut screen, p1s, p2s, p3s);

        // Copy screenbuffer to texture
        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            buffer.copy_from_slice(screen.buffer.as_ref());
        })?;

        // Blit
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

fn clip(p: (i32, i32), s: (i32, i32)) -> (i32, i32) {
    (i32::min(i32::max(0, p.0), s.0-1),
     i32::min(i32::max(0, p.1), s.1-1))
}

fn perspective(p: Vec4f) -> Vec4f {
    Vec4f::new(p.x / p.z, p.y / p.z, p.z, 1.0)
}

fn screenspace(p: Vec4f) -> Vec4f {
    let w = 1.33;
    let h = 1.0;
    Vec4f::new(p.x + 0.5 * w, p.y + 0.5 * h, p.z, 1.0)
}

fn pixelspace(p: Vec4f) -> Vec4f {
    Vec4f::new(p.x * 400.0, p.y * 300.0, p.z, 1.0)
}