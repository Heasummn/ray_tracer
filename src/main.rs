extern crate image;

mod camera;
mod ray;
mod vec3;

fn main() {
    let width: u32 = 1600;
    let height: u32 = 800;
    let mut dynamic_image = image::DynamicImage::new_rgb8(width, height);
    let imgbuf = dynamic_image.as_mut_rgb8().unwrap();

    let camera = camera::Camera::new(2, 1, 90.0);

    let ray = camera.get_ray(0, 0);

    println!("direction: {:?}", ray.direction);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let data = vec3::Vec3 {
            x: (x as f64 / width as f64),
            y: ((height - y) as f64) / height as f64,
            z: 0.2,
        } * 255.0;

        //println!("{:#?}", data);
        *pixel = data.into();
    }

    imgbuf.save("output.png").unwrap();
}
