use std::io;

use sphere::Sphere;

use crate::camera::Camera;
use crate::types::{Color, Point3, Ray};
use crate::utils::{Logger, Timer};

mod camera;
mod sphere;
mod types;
mod utils;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn color(ray: &Ray, sphere: &Sphere) -> Color {
    if sphere.hit(ray).is_some() {
        return Color::new(1.0, 0.0, 0.0);
    }

    let a = 0.5 * (ray.direction.y + 1.0);
    ((1.0 - a) * Color::new(1.0, 1.0, 1.0).to_vec() + a * Color::new(0.5, 0.7, 1.0).to_vec()).into()
}

fn main() {
    let mut log = Logger::new(io::stderr());
    let mut timer = Timer::start();

    let image_width = 400;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;

    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), 1.0).with_viewport_size(
        2.0,
        image_width,
        image_height,
    );

    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        log.progress_line(image_height - j).flush();
        for i in 0..image_width {
            let pixel_center = camera.pixel_center(i, j);
            let ray_direction = pixel_center.to_vec() - camera.center().to_vec();
            let ray = Ray::new(camera.center(), ray_direction);

            let pixel_color = color(&ray, &sphere);
            pixel_color.write(io::stdout());
        }
    }
    timer.stop();

    log.done().ln().elapsed(&timer).ln();
}
