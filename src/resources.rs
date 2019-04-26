extern crate image;
use image::*;

use crate::draw::*;

pub fn load_texture(path: String) -> Result<Vec<Color>,String> {
    let img = image::open(path).map_err(|e| e.to_string())?;

    let dims = img.dimensions();
    println!("image dimensions: {:?}", dims);

    let mut tex: Vec<Color> = Vec::with_capacity((dims.0 * dims.1) as usize);

    for x in 0..dims.0 {
        for y in 0..dims.1 {
            let c = img.get_pixel(x, y);
            tex.push(Color::new(c[0], c[1], c[2]));
        }
    }

    Ok(tex)
}