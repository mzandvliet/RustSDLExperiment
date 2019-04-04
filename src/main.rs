
extern crate sdl2;
extern crate gl;

use sdl2::rect::{Rect};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator};
use std::{thread, time};

/* Todo:
    - Start writing some simple game rendering algorithms

    On optimization:

    The loops we're using to draw pixels into buffer, and to copy buffer, are hella slow (without --release, anyway)
    https://stackoverflow.com/questions/47542438/does-rusts-array-bounds-checking-affect-performance
    https://llogiq.github.io/2017/06/01/perf-pitfalls.html

    - Bounds checking might be killing it
    - nested for-x for-y is cache-incoherent
    - with a (u8,u8,u8) tuple for color you can divide amount of array accesses by 3 (can write as iterator?)
    - Loop is a slow way to do a bitwise copy, can we not just memcpy to the texture while having it locked?
    - Compile with optimization flag (--release) OH THAT MAKES A HUGE DIFFERENCE!
    - For high res renders, we need to ensure screenbuffer memory layout remains cache friendly. For intermediate
    calculations we might want to use morton order or SFCs to keep things coherent.
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
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Our screen buffer
    let screen_buffer: Box<[u8]> = Box::new([0; SCREEN_BUFF_SIZE]);
    let mut screen = Screen {
        buffer: screen_buffer,
        width: WIDTH as usize,
        height: HEIGHT as usize,
    };
    
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
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Our rendering logic
        draw_gradient(&mut screen);
        draw_line(&mut screen, (10, 10), (110, 10));
        draw_line(&mut screen, (110, 10), (110, 110));
        draw_line(&mut screen, (110, 110), (10, 110));
        draw_line(&mut screen, (10, 110), (10, 10));

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

fn draw_gradient(screen: &mut Screen) {
    let pitch = screen.width * 3;
    for y in 0..screen.height {
        for x in 0..screen.width {
            let offset = y * pitch + x * 3;
            screen.buffer[offset] = x as u8;
            screen.buffer[offset +1] = y as u8;
            screen.buffer[offset +2] = 0;
        }
    }
}

// Bresenham
fn draw_line(screen: &mut Screen, a: (i32, i32), b: (i32, i32)) {
    let mut x0: i32 = a.0;
    let mut y0: i32 = a.1;

    let x1: i32 = b.0;
    let y1: i32 = b.1;

    let dx: i32 = (x1-x0).abs();
    let sx: i32 = if x0<x1 { 1 } else { -1 };
    let dy: i32 = -(y1-y0).abs();
    let sy: i32 = if y0<y1 { 1 } else { -1 };

    let mut err = dx+dy;
    let mut e2: i32;

    let c = math::Color::new(255,255,255);

    loop {
        set_pixel(screen, x0 as usize, y0 as usize, c);
        e2 = 2 * err;
        if e2 >= dy {
            if x0 == x1 { break }
            err += dy; x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 { break }
            err += dx; y0 += sy;
        }
    }
}

fn set_pixel(screen: &mut Screen, x: usize, y: usize, c: math::Color) {
    //println!("settting pixel: [{},{}]", x, y);

    let pitch = screen.width * 3;
    let offset = y * pitch + x * 3;

    // Todo: given Rust does bounds checks, it *might* be faster to writing using (u8,u8,u8) or (u8,u8,u8,u8) tuples
    screen.buffer[offset] = c.r;
    screen.buffer[offset+1] = c.g;
    screen.buffer[offset+2] = c.b;
}

struct Screen {
    pub buffer: Box<[u8]>,
    pub width: usize,
    pub height: usize,
}