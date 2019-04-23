
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
    Define a cube using 3d points X
    Transform it using matrices X
    Bring into camera space using inverse cam matrix X
    Draw using bresenham X
    Project to 2d screen using a proper projection matrix (with configurable FOV)

    Use From/Into trait impls to get around all the explicit casting
    Pass by immutable reference more, instead of by copy. This is not C#, mister.

    Then:
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
        4, 5, 6, 
        4, 6, 7,

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
        0, 4, 7, 
        0, 7, 3);

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

        // Clear
        draw::clear(&mut screen);
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Cam setup
        let cam_mat = Mat4x4f::translation(0.0, 0.0, -8.0);
        let cam_mat_inverse = cam_mat.inverse();

        // Let's draw our cube

        // rotate and translate it in world space
        let tri_mat = Mat4x4f::translation(0.0, f32::sin(time * 0.5), 0.0) * Mat4x4f::rotation_y(time * 1.3456);
        
        // draw all tris in sequence
        for i in 0..12 {
            draw_triangle(
                &verts[tris[i*3 + 0]],
                &verts[tris[i*3 + 1]],
                &verts[tris[i*3 + 2]],
                &tri_mat,
                &cam_mat_inverse,
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

fn draw_triangle(p1: &Vec4f, p2: &Vec4f, p3: &Vec4f, obj_mat: &Mat4x4f, cam_inv: &Mat4x4f, screen: &mut draw::Screen) {
    // Todo: split this into multiple stages, of course, and
    // loop over a list of points instead

    let p1 = *obj_mat * *p1;
    let p2 = *obj_mat * *p2;
    let p3 = *obj_mat * *p3;

    // World to camera space
    let p1 = *cam_inv * p1;
    let p2 = *cam_inv * p2;
    let p3 = *cam_inv * p3;

    // Projection as separate steps
    // todo: encode these steps in an actual 4x4 projection matrix
    let p1 = perspective_divide(p1);
    let p2 = perspective_divide(p2);
    let p3 = perspective_divide(p3);

    let p1 = to_screenspace(p1);
    let p2 = to_screenspace(p2);
    let p3 = to_screenspace(p3);

    let p1 = to_pixelspace(p1);
    let p2 = to_pixelspace(p2);
    let p3 = to_pixelspace(p3);

    let screen_dims = (screen.width as i32, screen.height as i32);

    let p1s = clip_line((p1.x as i32, p1.y as i32), screen_dims);
    let p2s = clip_line((p2.x as i32, p2.y as i32), screen_dims);
    let p3s = clip_line((p3.x as i32, p3.y as i32), screen_dims);

    // println!("{:?}, {:?}, {:?}", p1s, p2s, p3s);

    draw::triangle(screen, p1s, p2s, p3s);
}

/*
Todo: When a line is fully off screen, don't draw it
If partially on screen, clip the line properly, without changing its geometry

For now, I'm just nudging the points into valid screen bounds
*/
fn clip_line(p: (i32, i32), s: (i32, i32)) -> (i32, i32) {
    (i32::min(i32::max(0, p.0), s.0-1),
     i32::min(i32::max(0, p.1), s.1-1))
}

fn perspective_divide(p: Vec4f) -> Vec4f {
    Vec4f::new(p.x / p.z, p.y / p.z, p.z, 1.0)
}

// Todo: don't hardcode the resolution
fn to_screenspace(p: Vec4f) -> Vec4f {
    let w = 1.33;
    let h = 1.0;
    Vec4f::new((p.x + 0.5 * w) / w, p.y + 0.5 * h, p.z, 1.0)
}

fn to_pixelspace(p: Vec4f) -> Vec4f {
    // Note, we're inverting y here
    Vec4f::new(p.x * 400.0, 300.0 - p.y * 300.0, p.z, 1.0)
}