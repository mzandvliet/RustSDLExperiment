/*
    Todo:

    - Create types for easier manipulation of screen points
*/

use crate::linalg::*;

pub struct Screen {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn white() -> Color {
        Color::new(255, 255, 255)
    }

    pub fn black() -> Color {
        Color::new(0, 0, 0)
    }

    pub fn red() -> Color {
        Color::new(255, 0, 0)
    }

    pub fn green() -> Color {
        Color::new(0, 255, 0)
    }

    pub fn blue() -> Color {
        Color::new(0, 0, 255)
    }
}

pub struct Mesh {
    pub verts: Vec<Vec4f>,
    pub tris: Vec<usize>,
    pub uvs: Vec<Vec2f>,
}

impl Mesh {
    pub fn new(verts: Vec<Vec4f>, tris: Vec<usize>, uvs: Vec<Vec2f>) -> Mesh {
        Mesh {
            verts: verts,
            tris: tris,
            uvs: uvs,
        }
    }
}

// Set an individual pixel's RGB color
// Todo: investigate access patterns, cache coherence. Using a space-
// filling curve memory layout might improve drawing to smaller areas.
pub fn set_pixel(screen: &mut Screen, x: usize, y: usize, c: &Color) {
    // println!("settting pixel: [{},{}]", x, y);

    // Todo: you don't want to be doing asserts this nested within core loops
    // what we need is line culling and clipping stages before we draw
    // assert!(x < screen.width);
    // assert!(y < screen.height);
    if x < screen.width && y < screen.height {
        let pitch = screen.width * 3;
        let offset = y * pitch + x * 3;

        // Todo: given that Rust does bounds checks, it *might* be faster to writing using (u8,u8,u8) or (u8,u8,u8,u8) tuples
        screen.buffer[offset] = c.r;
        screen.buffer[offset+1] = c.g;
        screen.buffer[offset+2] = c.b;
    }
}

// Bresenham line drawing algorithm, as per this wonderful paper:
// http://members.chello.at/~easyfilter/Bresenham.pdf
// Todo: assume points are valid screen coords, and don't do any
// bounds checks. Perform rigorous clipping earlier in code path.
pub fn line(screen: &mut Screen, a: (i32, i32), b: (i32, i32), color: &Color) {
    let mut x0: i32 = a.0;
    let mut y0: i32 = a.1;

    let x1: i32 = b.0;
    let y1: i32 = b.1;

    let dx: i32 =  (x1-x0).abs();
    let sx: i32 = if x0<x1 { 1 } else { -1 };
    let dy: i32 = -(y1-y0).abs();
    let sy: i32 = if y0<y1 { 1 } else { -1 };

    let mut err = dx+dy;
    let mut e2: i32;

    loop {
        set_pixel(screen, x0 as usize, y0 as usize, color);
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

pub fn triangle(
    p1: &Vec4f, p2: &Vec4f, p3: &Vec4f,
    uv1: &Vec2f, uv2: &Vec2f, uv3: &Vec2f,
    tex: &Vec<Color>,
    obj_mat: &Mat4x4f, cam_inv: &Mat4x4f, cam_proj: &Mat4x4f,
    screen: &mut Screen) {
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

    // backface culling
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

        // let shaded_color = draw::Color::new((255.0 * l_dot_n) as u8, (100.0 * l_dot_n) as u8, (150.0 * l_dot_n) as u8);
        // let wire_color = draw::Color::new(255, 255, 255);
        // draw::triangle_solid(screen, &p1, &p2, &p3, &shaded_color);
        // draw::triangle_wired(screen, &p1, &p2, &p3, &wire_color);

        triangle_textured(
            screen,
            &p1, &p2, &p3,
            uv1, uv2, uv3,
            tex,
            l_dot_n);
    }
}

pub fn triangle_wired(screen: &mut Screen, a: &Vec2f, b: &Vec2f, c: &Vec2f, color: &Color) {
    let screen_dims = (screen.width as i32, screen.height as i32);
    let a = to_pixelspace(&a, &screen_dims);
    let b = to_pixelspace(&b, &screen_dims);
    let c = to_pixelspace(&c, &screen_dims);

    let a = clip_point((a.0, a.1), screen_dims);
    let b = clip_point((b.0, b.1), screen_dims);
    let c = clip_point((c.0, c.1), screen_dims);

    line(screen, a, b, color);
    line(screen, b, c, color);
    line(screen, c, a, color);
}

pub fn triangle_solid(screen: &mut Screen, a: &Vec2f, b: &Vec2f, c: &Vec2f, color: &Color) {
    let screen_dims = (screen.width as i32, screen.height as i32);

    // We generate a screen-pixel-space bounding box around the triangle
    // to limit the region of pixels tested against the triangle
    let a_s = to_pixelspace(a, &screen_dims);
    let b_s = to_pixelspace(b, &screen_dims);
    let c_s = to_pixelspace(c, &screen_dims);

    let a_s = clip_point((a_s.0, a_s.1), screen_dims);
    let b_s = clip_point((b_s.0, b_s.1), screen_dims);
    let c_s = clip_point((c_s.0, c_s.1), screen_dims);

    let aabb = get_aabb(vec!(a_s,b_s,c_s), (screen.width as i32, screen.height as i32));

    // Loop over bounded pixels
    for x in (aabb.0).0..(aabb.1).0 {
        for y in (aabb.0).1..(aabb.1).1 {
            // Transform pixel position into camera space. If inside cam-space triangle, draw it.
            let pix_camspace = to_camspace(&(x as i32, y as i32), &screen_dims);

            let area = signed_area(&a, &b, &c);
            let w0 = signed_area(&a, &b, &pix_camspace) / area;
            let w1 = signed_area(&b, &c, &pix_camspace) / area;
            let w2 = signed_area(&c, &a, &pix_camspace) / area;

            let mut inside: bool = true;

            inside &= w0 > 0.0;
            inside &= w1 > 0.0;
            inside &= w2 > 0.0;

            if inside {
                let color = blend_color(Color::red(), Color::green(), Color::blue(), w0, w1, w2);
                set_pixel(screen, x as usize, y as usize, &color);
            }
        }
    }
}

pub fn triangle_textured(
    screen: &mut Screen,
    a: &Vec2f, b: &Vec2f, c: &Vec2f,
    a_uv: &Vec2f, b_uv: &Vec2f, c_uv: &Vec2f,
    tex: &Vec<Color>,
    l_dot_n: f32) {
    let screen_dims = (screen.width as i32, screen.height as i32);

    // We generate a screen-pixel-space bounding box around the triangle
    // to limit the region of pixels tested against the triangle
    let a_s = to_pixelspace(&a, &screen_dims);
    let b_s = to_pixelspace(&b, &screen_dims);
    let c_s = to_pixelspace(&c, &screen_dims);

    let a_s = clip_point((a_s.0, a_s.1), screen_dims);
    let b_s = clip_point((b_s.0, b_s.1), screen_dims);
    let c_s = clip_point((c_s.0, c_s.1), screen_dims);

    let aabb = get_aabb(vec!(a_s,b_s,c_s), (screen.width as i32, screen.height as i32));

    // Loop over bounded pixels
    for x in (aabb.0).0..(aabb.1).0 {
        for y in (aabb.0).1..(aabb.1).1 {
            // Transform pixel position into camera space. If inside cam-space triangle, draw it.
            let pix_camspace = to_camspace(&(x as i32, y as i32), &screen_dims);

            let area = signed_area(&a, &b, &c);
            let w_c = signed_area(&a, &b, &pix_camspace) / area;
            let w_a = signed_area(&b, &c, &pix_camspace) / area;
            let w_b = signed_area(&c, &a, &pix_camspace) / area;

            let mut inside: bool = true;

            inside &= w_c > 0.0;
            inside &= w_a > 0.0;
            inside &= w_b > 0.0;

            if inside {
                let uv = *a_uv * w_a + *b_uv * w_b + *c_uv * w_c;
                let uv_scr = ((uv.x * 63.999) as usize, ((1.0 - uv.y) * 63.999) as usize);

                if uv_scr.0 > 63 || uv_scr.1 > 63 {
                    println!("uv: {:?},", uv);
                    println!("a {:?}, b {:?}, c {:?}, p {:?}", a, b, c, pix_camspace);
                    println!("area {}, wa {}, wb {}, wc {}", area, w_a, w_b, w_c);
                } else {
                    let albedo = tex[uv_scr.0 * 64 + uv_scr.1];
                    let brightness = 0.1 + 0.9 * l_dot_n;
                    let shaded_color = Color::new(
                        (albedo.r as f32 * brightness) as u8,
                        (albedo.g as f32 * brightness) as u8,
                        (albedo.b as f32 * brightness) as u8);

                    set_pixel(screen, x as usize, y as usize, &shaded_color);
                }
            }
        }
    }
}

fn to_pixelspace(point: &Vec2f, screen_dims: &(i32, i32)) -> (i32,i32) {
    (screen_dims.0 / 2 + (point.x * screen_dims.0 as f32) as i32,
     screen_dims.1 / 2 - (point.y * screen_dims.1 as f32) as i32) // Note, we're inverting y here
}

fn to_camspace(screen_point: &(i32,i32), screen_dims: &(i32,i32)) -> Vec2f {
    Vec2f {
        x: (screen_point.0 - screen_dims.0 / 2) as f32 / screen_dims.0 as f32,
        y: (screen_dims.1 - screen_point.1 - screen_dims.1 / 2) as f32 / screen_dims.1 as f32, // Note, we're inverting y here
    }
}

/*
Todo: When a line is fully off screen, don't draw it
If partially on screen, clip the line properly, without changing its geometry

For now, I'm just nudging the points into valid screen bounds
*/
fn clip_point(point: (i32, i32), screen_dims: (i32, i32)) -> (i32, i32) {
    (i32::min(i32::max(0, point.0), screen_dims.0-1),
     i32::min(i32::max(0, point.1), screen_dims.1-1))
}

fn get_aabb(points: Vec<(i32,i32)>, screen_dims: (i32, i32)) -> ((i32,i32), (i32,i32)){
    let mut x_min: i32 = screen_dims.0;
    let mut y_min: i32 = screen_dims.1;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    for p in points.iter() {
        if (p.0) < x_min {x_min = p.0;}
        if (p.1) < y_min {y_min = p.1;}
        if (p.0) > x_max {x_max = p.0;}
        if (p.1) > y_max {y_max = p.1;}
    }

    // bounding box min/max points, padded with extra pixel
    // Todo: if we did the bounding box in camera space, we wouldn't need to pad like this
    ((x_min-1, y_min-1), (x_max+1, y_max+1)) 
}

fn signed_area(a: &Vec2f, b: &Vec2f, p: &Vec2f) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

fn blend_color(a: Color, b: Color, c: Color, w0: f32, w1: f32, w2: f32) -> Color {
    Color::new(
        ((a.r as f32 * w0) + (b.r as f32 * w1) + (c.r as f32 * w2)) as u8,
        ((a.g as f32 * w0) + (b.g as f32 * w1) + (c.g as f32 * w2)) as u8,
        ((a.b as f32 * w0) + (b.b as f32 * w1) + (c.b as f32 * w2)) as u8,
    )
}

// Bresenham-style circle drawing algorithm, as per this wonderful paper:
// http://members.chello.at/~easyfilter/Bresenham.pdf
pub fn circle(screen: &mut Screen, a: (i32, i32), radius: i32, color: &Color) {
    let mut x: i32  = -radius;
    let mut y: i32 = 0;
    let mut err = 2 - 2 * radius;

    loop {
        set_pixel(screen, (a.0-x) as usize, (a.1+y) as usize, color);
        set_pixel(screen, (a.0-y) as usize, (a.1-x) as usize, color);
        set_pixel(screen, (a.0+x) as usize, (a.1-y) as usize, color);
        set_pixel(screen, (a.0+y) as usize, (a.1+x) as usize, color);
        let r = err;
        if r <= y { y+=1; err += y*2+1; }
        if r > 0 || err > y { x+=1; err += x*2+1; }

        if x > 0 {
            break;
        }
    }
}

pub fn clear(screen: &mut Screen) {
    let pitch = screen.width * 3;
    for y in 0..screen.height {
        for x in 0..screen.width {
            let offset = y * pitch + x * 3;
            screen.buffer[offset] = 0;
            screen.buffer[offset +1] = 0;
            screen.buffer[offset +2] = 0;
        }
    }
}