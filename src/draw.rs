/*
    Todo:

    - Create types for points
*/

pub struct Screen {
    pub buffer: Box<[u8]>,
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
}

pub fn set_pixel(screen: &mut Screen, x: usize, y: usize, c: Color) {
    // println!("settting pixel: [{},{}]", x, y);

    // Todo: you don't want to be doing asserts this nested within core loops
    // what we need is line cullign and clipping stages before we draw
    assert!(x < screen.width);
    assert!(y < screen.height);

    let pitch = screen.width * 3;
    let offset = y * pitch + x * 3;

    // Todo: given that Rust does bounds checks, it *might* be faster to writing using (u8,u8,u8) or (u8,u8,u8,u8) tuples
    screen.buffer[offset] = c.r;
    screen.buffer[offset+1] = c.g;
    screen.buffer[offset+2] = c.b;
}

// Bresenham
pub fn line(screen: &mut Screen, a: (i32, i32), b: (i32, i32)) {
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

    let c = Color::new(255,255,255);

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

pub fn circle(screen: &mut Screen, a: (i32, i32), radius: i32) {
    let mut x: i32  = -radius;
    let mut y: i32 = 0;
    let mut err = 2 - 2 * radius;

    let c = Color::new(255,255,255);

    loop {
        set_pixel(screen, (a.0-x) as usize, (a.1+y) as usize, c);
        set_pixel(screen, (a.0-y) as usize, (a.1-x) as usize, c);
        set_pixel(screen, (a.0+x) as usize, (a.1-y) as usize, c);
        set_pixel(screen, (a.0+y) as usize, (a.1+x) as usize, c);
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

pub fn gradient(screen: &mut Screen) {
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

/*
    First thing to notice; Where the C++ uses implicit casing between numerical
    types, Rust does no such thing.

    If I try:
    a:i64 = b: i64 * c: i32;

    Compiler goes: you cannot subtract those, they are different types

    Next: floating point

    Probably because there was no easily available rational type around,
    and because maybe some of the numbers get really big?

    We should investigate ways to use fractional arithmetic here without
    using floats. THERE IS SO MUCH IMPLICIT CASTING IN THIS ALG, made
    explicit by Rust not allowing it by default.

    Porting this code is taking ages, and its not getting pretties. Lots to
    learn workflow-wise.

    Todo: port the simpler algs first

    Note: From and Into traits might work well here
*/

// fn draw_aa_dcb(screen: &mut Screen, mut a: (i64, i64), mut b: (i64,i64), mut c: (i64, i64)) {
//     let mut sx = (c.0-b.0) as i64; let mut sy = (c.1-b.1) as i64;
//     let mut xx: i64 = (a.0-b.0) as i64; let mut yy: i64 = (a.1-b.1) as i64; let mut xy: i64;
//     let mut dx: f64; let mut dy: f64; let mut err: i64; let mut cur: f64 = (xx * sy as i64 - yy * sx as i64) as f64;

//     println!("xx {}, sx {}, yy {}, sy {}", xx, sx, yy, sy);

//     assert!(xx*sx >= 0 && yy*sy >= 0);// gradient may not change sign

//     if sx*sx + sy * sy > xx*xx+yy*yy {
//         c.0 = a.0; a.0 = sx+b.0; c.1 = a.1; a.1 = sy+b.1; cur = -cur; /* swap P0 P2 */
//     }

//     if cur != 0.0
//     {                                                                    /* no straight line */
//         sx = if a.0 < c.0 {1} else {-1};                             
//         xx += sx; xx *= sx;                                              /* x step direction */
//         sy = if a.1 < c.1 {1} else {-1};
//         yy += sy; yy *= sy;                                              /* y step direction */
//         xy = 2*xx*yy; xx *= xx; yy *= yy;                                /* differences 2nd degree */
//         if (cur * (sx*sy)as f64) < 0.0 {                                 /* negated curvature? */
//             xx = -xx; yy = -yy; xy = -xy; cur = -cur;
//         }
//         dx = (4.0*cur) * (sy*(b.0-a.0)+xx-xy) as f64;                    /* differences 1st degree */
//         dy = (4.0*cur) * (sx*(a.1-b.1)+yy-xy) as f64;
//         xx += xx; yy += yy; err = (dx+dy) as i64 + xy;                   /* error 1st step */
//         loop {                              
//             cur =  (dx + xy as f64).min(-xy as f64 - dy);
//             let ed = (dx+xy as f64).max(-xy as f64 - dy);                /* approximate error distance */
//             let ed = ed+2.0*ed*cur*cur/(4.0*ed*ed+cur*cur); // was u8 / f64, did that lose fraction?
//             let pixel = 255;//(ed*((err as f64 - dx - dy - xy as f64).abs())) as u8;
//             set_pixel(screen, a.0 as usize, a.1 as usize, math::Color::new(pixel, pixel, pixel));          /* plot curve */
//             if a.0 == c.0 && a.1 == c.1 {break};/* last pixel -> curve finished */
//             b.0 = a.0; cur = dx-err as f64; b.1 = if ((2*err) as f64+ dy) < 0.0 {1} else {0};
//             if (2*err) as f64 +dx > 0.0 {                                    /* x step */
//                 let pixel = 255;//(ed*(err as f64-dy).abs()) as u8;
//                 if err as f64-dy < ed { set_pixel(screen, a.0 as usize,(a.1+sy) as usize, math::Color::new(pixel,pixel,pixel)) };
//                 dy += yy as f64;
//                 a.0 += sx; dx -= xy as f64; err += dy as i64;
//             }
//             if b.1 != 0 {                                              /* y step */
//                 if cur < ed {
//                     let pixel = 255;//(ed*cur.abs()) as u8;
//                     set_pixel(screen, (b.0+sx) as usize, a.1 as usize, math::Color::new(pixel,pixel,pixel));
//                 };
//                 dx += xx as f64;
//                 a.1 += sy; dy -= xy as f64; err += dx as i64; 
//             }

//             if dy < dx {
//                 break;
//             }
//         }          /* gradient negates -> close curves */
//     }
//     draw_line(screen, (a.0 as i32, a.1 as i32), (c.0 as i32, c.1 as i32));              /* plot remaining needle to end */
// }

