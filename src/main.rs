
extern crate sdl2;
extern crate gl;

use sdl2::rect::{Rect};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator};
use std::{thread, time};

/* Todo:
    - Optimize core pixel drawing process
    - Start writing some simple game rendering algorithms

    On optimization:

    The loops we're using to draw pixels into buffer, and to copy buffer, are hella slow
    https://stackoverflow.com/questions/47542438/does-rusts-array-bounds-checking-affect-performance
    https://llogiq.github.io/2017/06/01/perf-pitfalls.html

    - Bounds checking might be killing it
    - nested for-x for-y is cache-incoherent
    - with a (u8,u8,u8) tuple for color you can divide amount of array accesses by 3 (can write as iterator?)
    - Loop is a slow way to do a bitwise copy, can we not just memcpy to the texture while having it locked?
    - Compile with optimization flag (--release) OH THAT MAKES A HUGE DIFFERENCE!

*/


fn main() {
    do_game();
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
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Our screen buffer
    let mut screen_buffer: [u8; SCREEN_BUFF_SIZE] = [0; SCREEN_BUFF_SIZE];
    
    // Texture used to blit our screen buffer to canvas
    let texture_creator : TextureCreator<_> = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame : u32 = 0;

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
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Our rendering logic (TODO: This is hella slow)
        draw(&mut screen_buffer, WIDTH as usize, HEIGHT as usize);

        // Copy screenbuffer to texture (TODO: This is hella slow)
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for i in 0..SCREEN_BUFF_SIZE {
                buffer[i] = screen_buffer[i];
            }
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

fn draw(buffer: &mut [u8], width: usize, height: usize) {
    let pitch = width * 3;
    for y in 0..height {
        for x in 0..width {
            let offset = y * pitch + x * 3;
            buffer[offset] = x as u8;
            buffer[offset +1] = y as u8;
            buffer[offset +2] = 0;
        }
    }
}