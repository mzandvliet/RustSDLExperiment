
extern crate sdl2;
extern crate gl;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

/* Todo:

    - Let us efficiently draw individual pixels

    https://docs.rs/sdl2/0.32.1/sdl2/render/struct.Canvas.html
    Recommends *not* using canvas to draw individual pixels

    Maybe through a texture, which we then blit to canvas?
*/


fn main() {
    //hello_sdl();
    do_game();
}

fn do_game() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let width = 640;
    let height = 480;

    let window = video_subsystem
        .window("rust-sdl-demo", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator : TextureCreator<_> = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height).map_err(|e| e.to_string())?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..(height as usize) {
            for x in 0..(width as usize) {
                let offset = y * pitch + x * 3;
                buffer[offset] = x as u8;
                buffer[offset +1] = y as u8;
                buffer[offset +2] = 0;
            }
        }
    })?;  

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame : u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    //game.toggle_state();
                }
                _ => {}
            }
        }

        if frame >= 30 {
            //game.update();
            frame = 0;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Todo: render to texture, copy tex to canvas?
        // I want to draw individual pixels so I can bresenham, do triangle rasterization, whatever

        // get a texture from canvas' texture creator
        // use texture.update(rect, pixeldata) to write from game data to texture
        // use canvas.copy to blit texture to screen

        let screen_rect = Rect::new(0,0,width,height);
        canvas.copy(&texture, screen_rect, screen_rect)?;

        // canvas.set_draw_color(Color::RGB(255,0,0));
        // canvas.fill_rect(Rect::new(10,10, 400, 200))?;

        canvas.present();

        frame += 1;
    }

    Ok(())
}


// Hello SDL

fn hello_sdl() {
    println!("Hello, SDL!");

    let gl = find_sdl_gl_driver().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("My Window", 640, 480)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .index(gl)
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();

    unsafe {
        gl::ClearColor(0.6, 0.0, 0.8, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    canvas.present();
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}