
extern crate sdl2;
extern crate gl;

use sdl2::rect::{Rect};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator};
use std::{thread, time};

mod graphics;
use graphics::draw;

mod math;
use math::linalg;

/*
    Prototype:
    Define a cube using 3d points
    Project to 2d screen using a matrix
    Draw using bresenham

    Then:
    Implement  Lengyel's book, plus a rasterizer
*/


fn main() {
    do_game().unwrap();
}

fn do_game() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
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

    let _x = linalg::Vec2f::new(1.0,2.0);

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
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Our rendering logic
        // draw::gradient(&mut screen);
        draw::line(&mut screen, (10, 10), (110, 10));
        draw::line(&mut screen, (110, 10), (110, 110));
        draw::line(&mut screen, (110, 110), (10, 110));
        draw::line(&mut screen, (10, 110), (10, 10));

        draw::circle(&mut screen, (350, 250), 100);

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
        thread::sleep(delta_time);
    }

    Ok(())
}