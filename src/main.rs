extern crate image;

mod vec3;

fn main() {
    let width: u32 = 1600;
    let height: u32 = 800;
    let mut imgbuf = image::RgbImage::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let data = vec3::Vec3 {
            x: (x as f64 / width as f64),
            y: ((height - y) as f64) / height as f64,
            z: 0.2,
        } * 255.0;

        //println!("R: {}, G: {}, B: {}", r, g, b);
        *pixel = data.into();
    }

    imgbuf.save("output.png").unwrap();
}
