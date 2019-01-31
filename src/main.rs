extern crate image;

mod camera;
mod ray;
mod vec3;
mod geometry;

use geometry::{sphere, plane, intersectable::Intersectable};

fn main() {
    let width: u32 = 800;
    let height: u32 = 600;
    let mut dynamic_image = image::DynamicImage::new_rgb8(width, height);
    let imgbuf = dynamic_image.as_mut_rgb8().unwrap();

    let camera = camera::Camera::new(width, height, 90.0);
    let sphere = sphere::Sphere {
        position: vec3::Vec3 {
            x: -3.0,
            y: 1.0,
            z: -6.0,
        },
        radius: 2.0,
    };
    

    for x in 0..camera.width {
        for y in 0..camera.height {
            let ray = camera.get_ray(x, y);
            if sphere.intersects(&ray) {
                imgbuf.put_pixel(x, y, image::Rgb([255, 0, 0]))
            } else {
                imgbuf.put_pixel(x, y, image::Rgb([0, 0, 0]))
            }
        }
    }

    /*for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let data = vec3::Vec3 {
            x: (x as f64 / width as f64),
            y: ((height - y) as f64) / height as f64,
            z: 0.2,
        } * 255.0;
    
        //println!("{:#?}", data);
        *pixel = data.into();
    }*/

    imgbuf.save("output.png").unwrap();
}
