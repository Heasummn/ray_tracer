extern crate image;

fn main() {
    let width: u32 = 1600;
    let height: u32 = 800;
    let mut imgbuf = image::RgbImage::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = (((height - y) as f32 / height as f32) * 255.0) as u8;
        let b = (255.0 * 0.2) as u8;

        //println!("R: {}, G: {}, B: {}", r, g, b);
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("output.png").unwrap();
}
