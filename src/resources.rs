extern crate image;
use image::*;

use crate::draw::*;
use crate::linalg::*;

pub fn load_texture(path: String) -> Result<Vec<Color>,String> {
    let img = image::open(path).map_err(|e| e.to_string())?;
    let dims = img.dimensions();
    let mut tex: Vec<Color> = Vec::with_capacity((dims.0 * dims.1) as usize);

    for x in 0..dims.0 {
        for y in 0..dims.1 {
            let c = img.get_pixel(x, y);
            tex.push(Color::new(c[0], c[1], c[2]));
        }
    }

    Ok(tex)
}

pub fn create_test_triangle() -> Mesh {
    // vert buffer
    let verts = vec!(
        Vec4f::new(-1.0, -1.0, -1.0, 1.0),
        Vec4f::new(-1.0,  1.0, -1.0, 1.0),
        Vec4f::new( 1.0,  1.0, -1.0, 1.0)
    );

    // index buffer
    let tris = vec!(
        0, 1, 2
    );

    let uvs = vec!(
        // front
        Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
    );

    Mesh::new(verts, tris, uvs)
}

pub fn create_cube() -> Mesh {
    // vert buffer
    let verts = vec!(
        Vec4f::new(-1.0, -1.0, -1.0, 1.0),
        Vec4f::new(-1.0,  1.0, -1.0, 1.0),
        Vec4f::new( 1.0,  1.0, -1.0, 1.0),
        Vec4f::new( 1.0, -1.0, -1.0, 1.0),
        Vec4f::new(-1.0, -1.0,  1.0, 1.0),
        Vec4f::new(-1.0,  1.0,  1.0, 1.0),
        Vec4f::new( 1.0,  1.0,  1.0, 1.0),
        Vec4f::new( 1.0, -1.0,  1.0, 1.0)
    );

    // index buffer
    let tris = vec!(
        // front
        0, 1, 2, 
        0, 2, 3,

        // back
        6, 5, 4, 
        7, 6, 4,

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
        7, 4, 0, 
        3, 7, 0
    );

    let uvs = vec!(
        // front
        Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
        Vec2f::new(0.0, 0.0), Vec2f::new(1.0, 1.0), Vec2f::new(1.0, 0.0),

        // back
         Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
        Vec2f::new(0.0, 0.0), Vec2f::new(1.0, 1.0), Vec2f::new(1.0, 0.0),

        // left
         Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
        Vec2f::new(0.0, 0.0), Vec2f::new(1.0, 1.0), Vec2f::new(1.0, 0.0),

        // right
         Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
        Vec2f::new(0.0, 0.0), Vec2f::new(1.0, 1.0), Vec2f::new(1.0, 0.0),
        
        // top
         Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
        Vec2f::new(0.0, 0.0), Vec2f::new(1.0, 1.0), Vec2f::new(1.0, 0.0),

        // bottom
        Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 1.0), Vec2f::new(1.0, 1.0), 
        Vec2f::new(1.0, 0.0), Vec2f::new(1.0, 1.0), Vec2f::new(0.0, 0.0),
    );

    Mesh::new(verts, tris, uvs)
}