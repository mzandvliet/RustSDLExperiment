/*
    Todo:

    - Features
        - Depth buffer for rendering multiple objects
        - Clipping geometry to frustum
        - Anti-aliasing by subdividing edge pixels
        - Back-face culling by checking sign of cross product of edges

    - Optimization
        - Texture and screenbuffer access, as those are biggest bottlenecks
            - Cache misses, I think
            - Accessing by [RGBA] instead of [R], [G], [B] would cut down on total
            accesses, though whether that helps with cache misses I don't know.
            - Tiled rendering
        - Block / tile rendering
            - Test corners first. If all corners lie in triangle, inside of block does too
                - large screen-space triangles benefit
            - Tiles with morton-order indexing could have better cache behaviour
        - Fixed point arithmetic

*/

extern crate float_cmp;

use crate::linalg::*;

pub struct Screen {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vec2i {
    x: i32,
    y: i32
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i {
            x: x,
            y: y,
        }
    }
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

    // Blend 3 colors using barycentric coordinates
    pub fn blend(a: Color, b: Color, c: Color, w0: f32, w1: f32, w2: f32) -> Color {
        Color::new(
            ((a.r as f32 * w0) + (b.r as f32 * w1) + (c.r as f32 * w2)) as u8,
            ((a.g as f32 * w0) + (b.g as f32 * w1) + (c.g as f32 * w2)) as u8,
            ((a.b as f32 * w0) + (b.b as f32 * w1) + (c.b as f32 * w2)) as u8,
        )
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
    assert!(x < screen.width);
    assert!(y < screen.height);

    let pitch = screen.width * 3;
    let offset = y * pitch + x * 3;

    // Todo: given that Rust does bounds checks, it *might* be faster to writing using (u8,u8,u8) or (u8,u8,u8,u8) tuples
    screen.buffer[offset] = c.r;
    // screen.buffer[offset+1] = c.g;
    // screen.buffer[offset+2] = c.b;
}

// Bresenham line drawing algorithm, as per this wonderful paper:
// http://members.chello.at/~easyfilter/Bresenham.pdf
// Todo: assume points are valid screen coords, and don't do any
// bounds checks. Perform rigorous clipping earlier in code path.
pub fn line(screen: &mut Screen, a: Vec2i, b: Vec2i, color: &Color) {
    let mut x0: i32 = a.x;
    let mut y0: i32 = a.y;

    let x1: i32 = b.x;
    let y1: i32 = b.y;

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
        let p1 = *cam_proj * p1;
        let p2 = *cam_proj * p2;
        let p3 = *cam_proj * p3;
        
        // Normalize x,y,z by w to get valid point
        let mut p1 = Vec4f::norm_by_w(&p1);
        let mut p2 = Vec4f::norm_by_w(&p2);
        let mut p3 = Vec4f::norm_by_w(&p3);

        // Store depth reciprocal in w for use in fragment stage
        p1.w = 1.0 / p1.w;
        p2.w = 1.0 / p2.w;
        p3.w = 1.0 / p3.w;

        triangle_textured(
            screen,
            &p1, &p2, &p3,
            uv1, uv2, uv3,
            tex,
            l_dot_n);

        // Wireframe
        // let wire_color = draw::Color::new(255, 255, 255);
        // draw::triangle_wired(screen, &p1, &p2, &p3, &wire_color);
    }
}

pub fn triangle_wired(screen: &mut Screen, a: &Vec4f, b: &Vec4f, c: &Vec4f, color: &Color) {
    let screen_dims = Vec2i::new(screen.width as i32, screen.height as i32);
    let a = to_pixelspace(&a, &screen_dims);
    let b = to_pixelspace(&b, &screen_dims);
    let c = to_pixelspace(&c, &screen_dims);

    let a = clip_point(&a, &screen_dims);
    let b = clip_point(&b, &screen_dims);
    let c = clip_point(&c, &screen_dims);

    line(screen, a, b, color);
    line(screen, b, c, color);
    line(screen, c, a, color);
}

pub fn triangle_textured(
    screen: &mut Screen,
    a: &Vec4f, b: &Vec4f, c: &Vec4f,
    a_uv: &Vec2f, b_uv: &Vec2f, c_uv: &Vec2f,
    tex: &Vec<Color>,
    l_dot_n: f32) {
    let screen_dims = Vec2i::new(screen.width as i32, screen.height as i32);

    // We generate a screen-pixel-space bounding box around the triangle
    // to limit the region of pixels tested against the triangle
    let a_s = to_pixelspace(&a, &screen_dims);
    let b_s = to_pixelspace(&b, &screen_dims);
    let c_s = to_pixelspace(&c, &screen_dims);
    let a_s = clip_point(&a_s, &screen_dims);
    let b_s = clip_point(&b_s, &screen_dims);
    let c_s = clip_point(&c_s, &screen_dims);

    let aabb = get_aabb(vec!(a_s,b_s,c_s), &screen_dims);

    // Loop over bounded pixels
    for x in (aabb.0).x..(aabb.1).x {
        for y in (aabb.0).y..(aabb.1).y {
            // Transform pixel position into camera space. If inside cam-space triangle, draw it.
            let pix_camspace = to_camspace(&Vec2i::new(x,y), &screen_dims);

            let area = signed_area(&a, &b, &c);
            let w_a = signed_area(&b, &c, &pix_camspace) / area;
            let w_b = signed_area(&c, &a, &pix_camspace) / area;
            let w_c = signed_area(&a, &b, &pix_camspace) / area;

            let edge_0 = *c - *a;
            let edge_1 = *a - *c;
            let edge_2 = *b - *a;

            let mut inside: bool = true;

            /*
            If all three edge tests are positive, or we're a pixel right
            on the edge of a top-left triangle, then we rasterize
            */
            
            test_topleft(&edge_0, w_a, &mut inside);
            test_topleft(&edge_1, w_b, &mut inside);
            test_topleft(&edge_2, w_c, &mut inside);

            if inside {
                // interpolate UV values with barycentric coordinates

                let z = 1.0 / (
                    a.w * w_a +
                    b.w * w_b +
                    c.w * w_c);

                let uv = 
                    (*a_uv * a.w) * w_a +
                    (*b_uv * b.w) * w_b +
                    (*c_uv * c.w) * w_c;

                let uv = uv * z;

                // transform UV values to texture space (taking care not to read out of bounds)
                // todo: get texture dimensions from texture, instead of hardcoding
                let uv_scr = ((uv.x * 63.999) as usize, ((1.0 - uv.y) * 63.999) as usize);

                // read from texture, without filtering
                // let albedo = tex[uv_scr.0 * 64 + uv_scr.1];
                let albedo = Color::blue();

                // shade pixel
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

/*
    Note:
    reference implementation, performance is not great
    working with float approx this way feels nasty (use fixed?)
    not having ternary statements makes this code lengthy
*/
fn test_topleft(edge: &Vec4f, w: f32, inside: &mut bool) {
     if approx_eq(w, 0.0) {
            // If lying on an edge, are we a top-left?
            *inside &= (approx_eq(edge.y, 0.0) && edge.x > 0.0) || edge.y > 0.0;
        } else {
            // If we're on the positive side of the half-plane defined by the edge
            *inside &= w > 0.0;
        }
}

fn approx_eq(a: f32, b: f32) -> bool {
    /* 
        Using library function that's fully compliant with floating point spec
        and is correct for every possible combination of values.

        This makes the function ***stupidly*** slow, adding whole miliseconds.
    */
    // a.approx_eq(&b, 2.0 * ::std::f32::EPSILON, 2)

    /*
        My much less correct implementation that still gets me results, at a
        small fraction of the cost.
    */
    (a.abs() - b.abs()).abs() < std::f32::EPSILON

    /*
        Todo: This approx_eq test, as used to determine whether a pixel lies on
        a triangle edge, is a prime candidate for fixed point logic.
    */
}

fn to_pixelspace(point: &Vec4f, screen_dims: &Vec2i) -> Vec2i {
    Vec2i::new(
        screen_dims.x / 2 + (point.x * screen_dims.x as f32) as i32,
        screen_dims.y / 2 - (point.y * screen_dims.y as f32) as i32) // Note, we're inverting y here
}

fn to_camspace(screen_point: &Vec2i, screen_dims: &Vec2i) -> Vec4f {
    Vec4f {
        x: (screen_point.x - screen_dims.x / 2) as f32 / screen_dims.x as f32,
        y: (screen_dims.y - screen_point.y - screen_dims.y / 2) as f32 / screen_dims.y as f32, // Note, we're inverting y here
        z: 0.0,
        w: 1.0
    }
}

fn project(p: &Vec4f) -> Vec4f {
    Vec4f::new(p.x / p.z, p.y / p.z, p.z, 1.0)
}

/*
Todo: When a line is fully off screen, don't draw it
If partially on screen, clip the line properly, without changing its geometry

For now, I'm just nudging the points into valid screen bounds
*/
fn clip_point(point: &Vec2i, screen_dims: &Vec2i) -> Vec2i {
    Vec2i::new(
        i32::min(i32::max(0, point.x), screen_dims.x-1),
        i32::min(i32::max(0, point.y), screen_dims.y-1))
}

// Calculate a screen-space bounding box for a given polygon/triangle
fn get_aabb(points: Vec<Vec2i>, screen_dims: &Vec2i) -> (Vec2i, Vec2i){
    let mut x_min: i32 = screen_dims.x;
    let mut y_min: i32 = screen_dims.y;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    for p in points.iter() {
        if (p.x) < x_min {x_min = p.x;}
        if (p.y) < y_min {y_min = p.y;}
        if (p.x) > x_max {x_max = p.x;}
        if (p.y) > y_max {y_max = p.y;}
    }

    // bounding box min/max points, padded with extra pixel
    // Todo: if we did the bounding box in camera space, we wouldn't need to pad like this
    (
        Vec2i::new(x_min-1, y_min-1),
        Vec2i::new(x_max+1, y_max+1)
    ) 
}

// Todo: only using these as Vec2, so can we make a From<Vec3> that returns same mem reinterpreted as Vec2?
fn signed_area(a: &Vec4f, b: &Vec4f, p: &Vec4f) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
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

/*
    Todo: the below are unused as of now. Still need to clip lines
    and triangles to the screen bounds...

    And use the Vec2i type for them, of course
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cam_pixel_space_conversion() {
        let screen_dims = Vec2i::new(400, 300);
        let cam_space = Vec4f::new(0.65, 0.45, 0.0, 1.0);
        let pix_space = to_pixelspace(&cam_space, &screen_dims);
        let cam_space_b = to_camspace(&pix_space, &screen_dims);

        assert_eq!(cam_space, cam_space_b);
    }

    #[test]
    fn test_approx_eq() {
        for i in -32..32 {
            let a = i as f32 + 0.0;
            let b = i as f32 + 0.00000001;

            assert!(approx_eq(a, b));
        }
    }
}