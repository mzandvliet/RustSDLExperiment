/*
    Todo:

    - Features
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

#![allow(dead_code)]

extern crate float_cmp;

use crate::linalg::*;

pub struct Screen {
    pub color: Vec<u8>,
    pub depth: Vec<f32>,
    pub width: usize,
    pub height: usize,
   
}

pub struct TileCache {
    pub fast_tiles: Vec<BoundingBox>,
    pub slow_tiles: Vec<BoundingBox>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        let color_buffer_size = width * height * 3;
        let depth_buffer_size = width * height;

        let color_buffer: Vec<u8> = vec![0; color_buffer_size];
        let depth_buffer: Vec<f32> = vec![1000.0; depth_buffer_size];

        Screen {
            color: color_buffer,
            depth: depth_buffer,
            width: width,
            height: height,
        }
    }
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    pub a: Vec4f,
    pub b: Vec4f,
    pub c: Vec4f,
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

// Todo: unsigned types
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub bl:  Vec2i, // bottom left
    pub tr: Vec2i, // top right
}

impl BoundingBox {
    pub fn iter(&self, step: i32) -> BoundingBoxIterator {
        BoundingBoxIterator {
            bounds: *self,
            step: step,
            x: self.bl.x,
            y: self.bl.y,
        }
    }
}

pub struct BoundingBoxIterator {
    pub bounds: BoundingBox,
    pub step: i32,
    x: i32,
    y: i32
}

impl Iterator for BoundingBoxIterator {
    type Item = BoundingBox;

    fn next(&mut self) -> Option<BoundingBox> {
        let result = Some(BoundingBox { 
            bl: Vec2i::new(
                self.x,
                self.y),
            tr: Vec2i::new(
                i32::min(self.x + self.step, self.bounds.tr.x),
                i32::min(self.y + self.step, self.bounds.tr.y)),
        });

        self.x += self.step;
        if self.x > self.bounds.tr.x {
            self.x = self.bounds.bl.x;
            self.y += self.step;
        }

        if self.y >= self.bounds.tr.y {
            None
        } else {
            result
        }
    }
}

// Set an individual pixel's RGB color
// Todo: investigate access patterns, cache coherence. Using a space-
// filling curve memory layout might improve drawing to smaller areas. (for bresenham)
// or: use unsafe code, render a horizontal strip by pointer increment (for raster)
pub fn set_color(screen: &mut Screen, x: usize, y: usize, c: &Color) {
    // println!("settting pixel: [{},{}]", x, y);

    // Todo: you don't want to be doing asserts this nested within core loops
    // what we need is line culling and clipping stages before we draw
    assert!(x < screen.width);
    assert!(y < screen.height);

    let stride = screen.width * 3;
    let offset = y * stride + x * 3;

    // Todo: given that Rust does bounds checks, it *might* be faster to writing using (u8,u8,u8) or (u8,u8,u8,u8) tuples
    screen.color[offset+0] = c.r;
    screen.color[offset+1] = c.g;
    screen.color[offset+2] = c.b;
}

pub fn set_depth(screen: &mut Screen, x: usize, y: usize, d: f32) {
    assert!(x < screen.width);
    assert!(y < screen.height);

    let stride = screen.width;
    let offset = y * stride + x;

    screen.depth[offset] = d;
}

pub fn get_depth(screen: &mut Screen, x: usize, y: usize) -> f32 {
    assert!(x < screen.width);
    assert!(y < screen.height);

    let pitch = screen.width;
    let offset = y * pitch + x;

    screen.depth[offset]
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
        set_color(screen, x0 as usize, y0 as usize, color);
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

pub fn draw_mesh(mesh: &Mesh, tex: &Vec<Color>, transform: &Mat4x4f, cam: &Mat4x4f, screen: &mut Screen, tile_cache: &mut TileCache) {
    let verts = &mesh.verts;
    let tris = &mesh.tris;
    let uvs = &mesh.uvs;

    let num_tris = tris.len() / 3;
        for i in 0..num_tris {
            triangle(
                screen,
                tile_cache,
                &verts[tris[i*3 + 0]],
                &verts[tris[i*3 + 1]],
                &verts[tris[i*3 + 2]],
                &uvs[i*3 + 0],
                &uvs[i*3 + 1],
                &uvs[i*3 + 2],
                tex,
                transform,
                cam);
        }
}

pub fn triangle(
    screen: &mut Screen,
    tile_cache: &mut TileCache,
    p1: &Vec4f, p2: &Vec4f, p3: &Vec4f,
    uv1: &Vec2f, uv2: &Vec2f, uv3: &Vec2f,
    tex: &Vec<Color>,
    obj_mat: &Mat4x4f, cam: &Mat4x4f,) {

    // Todo: 
    // - split this into multiple stages, of course, and
    // - loop over a list of points instead

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

        // Project from world to cam space
        let p1 = *cam * p1;
        let p2 = *cam * p2;
        let p3 = *cam * p3;
        
        // Normalize x,y,z by w to get valid point
        let mut p1 = Vec4f::norm_by_w(&p1);
        let mut p2 = Vec4f::norm_by_w(&p2);
        let mut p3 = Vec4f::norm_by_w(&p3);

        // Store depth reciprocal in w for use in fragment stage
        p1.w = 1.0 / p1.w;
        p2.w = 1.0 / p2.w;
        p3.w = 1.0 / p3.w;

        // Todo: use triangles from earlier in the chain
        let tri = Triangle {
            a: p1,
            b: p2,
            c: p3,
        };

        triangle_textured(
            screen,
            tile_cache,
            &tri,
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
    tile_cache: &mut TileCache,
    tri: &Triangle,
    a_uv: &Vec2f, b_uv: &Vec2f, c_uv: &Vec2f,
    tex: &Vec<Color>,
    l_dot_n: f32) {

    let screen_dims = Vec2i::new(screen.width as i32, screen.height as i32);

    // println!("{:?}", to_camspace(&Vec2i::new(screen_dims.x,screen_dims.y), &screen_dims));

    // We generate a screen-pixel-space bounding box around the triangle
    // to limit the region of pixels tested against the triangle
    let a_s = to_pixelspace(&tri.a, &screen_dims);
    let b_s = to_pixelspace(&tri.b, &screen_dims);
    let c_s = to_pixelspace(&tri.c, &screen_dims);
    let a_s = clip_point(&a_s, &screen_dims);
    let b_s = clip_point(&b_s, &screen_dims);
    let c_s = clip_point(&c_s, &screen_dims);
    let bounds = get_bounds(vec!(a_s,b_s,c_s), &screen_dims);

    let edges = Triangle {
        a: tri.c - tri.a,
        b: tri.a - tri.c,
        c: tri.b - tri.a,
    };

    let tri_area_inv = 1.0 / signed_area(&tri.a, &tri.b, &tri.c);

    // Precalculate per-pixel barycentric coordinate steps in x and y
    // This is used for optimized edge function calculations, which
    // are linear in x and y.
    let step_x = (1.0 / screen_dims.x as f32) * tri_area_inv;
    let step_y = (1.0 / screen_dims.y as f32) * tri_area_inv;

    let bary_step_x = Vec3f::new(
        signed_area_step_x(step_x, &tri.b, &tri.c),
        signed_area_step_x(step_x, &tri.c, &tri.a),
        signed_area_step_x(step_x, &tri.a, &tri.b),
    );

    let bary_step_y = Vec3f::new(
        signed_area_step_y(step_y, &tri.b, &tri.c),
        signed_area_step_y(step_y, &tri.c, &tri.a),
        signed_area_step_y(step_y, &tri.a, &tri.b),
    );

    // Full barycentric coordinate calculation for bottom-left pixel coordinate
    let pix_camspace_bl = screen_to_camspace(&Vec2i::new(bounds.bl.x,bounds.bl.y), &screen_dims);

    let bary_bl = Vec3f::new(
        signed_area(&tri.b, &tri.c, &pix_camspace_bl) * tri_area_inv,
        signed_area(&tri.c, &tri.a, &pix_camspace_bl) * tri_area_inv,
        signed_area(&tri.a, &tri.b, &pix_camspace_bl) * tri_area_inv
    );

    tile_cache.fast_tiles.clear();
    tile_cache.slow_tiles.clear();

    // Here we iterate over the bounding area of the triangle in n*n tiles
    // If all corners or a tile fall within a triangle, we can skip the
    // edge tests for all the pixels inside it.
    //
    // Todo: if none of the corners fall in the triangle, skip the tile
    // entirely
    for tile in bounds.iter(8) {
        match triangle_box_corner_overlaps(tri, &edges, &tile, &screen_dims) {
            4 => tile_cache.fast_tiles.push(tile),
            0...3 => tile_cache.slow_tiles.push(tile),
            _ => ()
        }
    }
    
    // let tile_count = ((bounds.tr.x - bounds.bl.x) / 8 + 1) * ((bounds.tr.y - bounds.bl.y) / 8 + 1);
    // println!("{}, {}, {}", tile_count, tile_cache.fast_tiles.len(), tile_cache.slow_tiles.len());

    for tile in &tile_cache.fast_tiles {
        // Fast path

        let mut bary_row = bary_bl +
            bary_step_y * (tile.bl.y-bounds.bl.y) as f32 +
            bary_step_x * (tile.bl.x-bounds.bl.x) as f32;

        for y in tile.bl.y..tile.tr.y {
            let mut bary = bary_row;

            for x in tile.bl.x..tile.tr.x {
                shade(
                    tri,
                    a_uv, b_uv, c_uv,
                    &bary,
                    tex,
                    screen,
                    l_dot_n,
                    x as usize,
                    y as usize
                );

                // Step barycentric coordinates 1 pixel along x
                bary = bary + bary_step_x;
            }

            // Step barycentric coordinates 1 pixel along y
            bary_row = bary_row + bary_step_y;
        }
    }

    for tile in &tile_cache.slow_tiles {
        // Slow path
            
        let mut bary_row = bary_bl +
            bary_step_y * (tile.bl.y-bounds.bl.y) as f32 +
            bary_step_x * (tile.bl.x-bounds.bl.x) as f32;

        for y in tile.bl.y..tile.tr.y {
            let mut bary = bary_row;

            for x in tile.bl.x..tile.tr.x {
                let mut inside: bool = true;

                /*
                If all three edge tests are positive, or we're a pixel right
                on the edge of a top-left triangle, then we rasterize
                */
                test_topleft(&edges.a, bary.x, &mut inside);
                test_topleft(&edges.b, bary.y, &mut inside);
                test_topleft(&edges.c, bary.z, &mut inside);

                if inside {
                    shade(
                        tri,
                        a_uv, b_uv, c_uv,
                        &bary,
                        tex,
                        screen,
                        l_dot_n,
                        x as usize,
                        y as usize
                    );
                }

                // Step barycentric coordinates 1 pixel along x
                bary = bary + bary_step_x;
            }

            // Step barycentric coordinates 1 pixel along y
            bary_row = bary_row + bary_step_y;
        }
    }
}

// Todo: separate z testing from shading
fn shade(
    tri: &Triangle,
    a_uv: &Vec2f, b_uv: &Vec2f, c_uv: &Vec2f,
    bary: &Vec3f,
    tex: &Vec<Color>,
    screen: &mut Screen,
    l_dot_n: f32, x: usize, y: usize) {

    // interpolate UV values with barycentric coordinates
    let z = 1.0 / (
        tri.a.w * bary.x +
        tri.b.w * bary.y +
        tri.c.w * bary.z);

    let curr_depth = get_depth(screen, x, y);

    if z < curr_depth {
        let uv = 
        *a_uv * tri.a.w * bary.x +
        *b_uv * tri.b.w * bary.y +
        *c_uv * tri.c.w * bary.z;

        let uv = uv * z;

        // transform UV values to texture space (taking care not to read out of bounds)
        // todo: get texture dimensions from texture, instead of hardcoding
        let uv_scr = ((uv.x * 63.999) as usize, ((1.0 - uv.y) * 63.999) as usize);

        // read from texture, without filtering
        let albedo = tex[uv_scr.0 * 64 + uv_scr.1];
        // let albedo = Color::blue();

        // shade pixel
        let brightness = 0.1 + 0.9 * l_dot_n;
        let shaded_color = Color::new(
            (albedo.r as f32 * brightness) as u8,
            (albedo.g as f32 * brightness) as u8,
            (albedo.b as f32 * brightness) as u8);

        set_color(screen, x as usize, y as usize, &shaded_color);
        set_depth(screen, x as usize, y as usize, z);
    }
}

fn triangle_box_corner_overlaps(tri: &Triangle, edges: &Triangle, bounds: &BoundingBox, screen_dims: &Vec2i) -> u8 {
    // Todo: pass in precalculated values
    let tri_area_inv = 1.0 / signed_area(&tri.a, &tri.b, &tri.c);

    let bot_left  = screen_to_camspace(&Vec2i::new(bounds.bl.x,bounds.bl.y), &screen_dims);
    let top_left  = screen_to_camspace(&Vec2i::new(bounds.bl.x,bounds.tr.y), &screen_dims);
    let bot_right = screen_to_camspace(&Vec2i::new(bounds.tr.x,bounds.bl.y), &screen_dims);
    let top_right = screen_to_camspace(&Vec2i::new(bounds.tr.x,bounds.tr.y), &screen_dims);

    let mut edge_overlap_count: u8 = 0;
    if is_point_inside_triangle(tri, &edges, tri_area_inv, &bot_left) { edge_overlap_count += 1; }
    if is_point_inside_triangle(tri, &edges, tri_area_inv, &top_left) { edge_overlap_count += 1; }
    if is_point_inside_triangle(tri, &edges, tri_area_inv, &bot_right) { edge_overlap_count += 1; }
    if is_point_inside_triangle(tri, &edges, tri_area_inv, &top_right) { edge_overlap_count += 1; }

    edge_overlap_count
}

// Todo: use the optimized way to calculate the barycentric coords
fn is_point_inside_triangle(tri: &Triangle, edges: &Triangle, tri_area_inv: f32, pix_camspace: &Vec4f) -> bool {
    let bary = Vec3f::new(
        signed_area(&tri.b, &tri.c, &pix_camspace) * tri_area_inv,
        signed_area(&tri.c, &tri.a, &pix_camspace) * tri_area_inv,
        signed_area(&tri.a, &tri.b, &pix_camspace) * tri_area_inv
    );

    let mut inside: bool = true;

    test_topleft(&edges.a, bary.x, &mut inside);
    test_topleft(&edges.b, bary.y, &mut inside);
    test_topleft(&edges.c, bary.z, &mut inside);

    inside
}

/*
    Note:
    reference implementation, performance is not great
    working with float approx this way feels nasty (use fixed?)
    not having ternary statements makes this code lengthy
    Giesen makes this into 3 >= checks on ints
*/
fn test_topleft(edge: &Vec4f, w: f32, inside: &mut bool) {
     if approx_eq(w, 0.0) {
            // If lying on an edge, are we a top-left?
            *inside &= (approx_eq(edge.y, 0.0) && edge.x >= 0.0) || edge.y >= 0.0;
        } else {
            // If we're on the positive side of the half-plane defined by the edge
            *inside &= w >= 0.0;
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
    f32::abs(a - b) < std::f32::EPSILON * 2.0

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

fn screen_to_camspace(screen_point: &Vec2i, screen_dims: &Vec2i) -> Vec4f {
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
fn get_bounds(points: Vec<Vec2i>, screen_dims: &Vec2i) -> BoundingBox {
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

    BoundingBox {
        bl: Vec2i::new(x_min-1, y_min-1),
        tr: Vec2i::new(x_max+1, y_max+1)
    }
}

// Todo: only using these as Vec2, so can we make a From<Vec3> that returns same mem reinterpreted as Vec2?
fn signed_area(a: &Vec4f, b: &Vec4f, p: &Vec4f) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

fn signed_area_step_x(step_x: f32, a: &Vec4f, b: &Vec4f) -> f32 {
    // Uh, these are the reverse of what I derived them to be. But Giesen has them this way. Why?
    // because of the y inversion I have elsewhere?
    -step_x * (a.y - b.y) 
}

fn signed_area_step_y(step_y: f32, a: &Vec4f, b: &Vec4f) -> f32 {
    step_y * (b.x - a.x)
}

// Bresenham-style circle drawing algorithm, as per this wonderful paper:
// http://members.chello.at/~easyfilter/Bresenham.pdf
pub fn circle(screen: &mut Screen, a: (i32, i32), radius: i32, color: &Color) {
    let mut x: i32  = -radius;
    let mut y: i32 = 0;
    let mut err = 2 - 2 * radius;

    loop {
        set_color(screen, (a.0-x) as usize, (a.1+y) as usize, color);
        set_color(screen, (a.0-y) as usize, (a.1-x) as usize, color);
        set_color(screen, (a.0+x) as usize, (a.1-y) as usize, color);
        set_color(screen, (a.0+y) as usize, (a.1+x) as usize, color);
        let r = err;
        if r <= y { y+=1; err += y*2+1; }
        if r > 0 || err > y { x+=1; err += x*2+1; }

        if x > 0 {
            break;
        }
    }
}

pub fn clear_color(screen: &mut Screen) {
    let pitch = screen.width * 3;
    for y in 0..screen.height {
        for x in 0..screen.width {
            let offset = y * pitch + x * 3;
            screen.color[offset] = 0;
            screen.color[offset +1] = 0;
            screen.color[offset +2] = 0;
        }
    }
}

pub fn clear_depth(screen: &mut Screen) {
    let pitch = screen.width;
    for y in 0..screen.height {
        for x in 0..screen.width {
            let offset = y * pitch + x;
            screen.depth[offset] = 1000.0;
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
        let cam_space_b = screen_to_camspace(&pix_space, &screen_dims);

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

    #[test]
    fn test_bounds() {
        let aabb = BoundingBox {
            bl: Vec2i::new(20,24),
            tr: Vec2i::new(126, 127),
        };

        for b in aabb.iter(8) {
            println!("{:?}", b);
        }
    }
}