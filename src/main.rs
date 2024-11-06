use std::io;

use render::{Fading, Renderer};

use crate::object::Hittable;
use crate::sphere::Sphere;
use crate::types::Point3;
use crate::utils::Logger;

mod camera;
mod object;
mod render;
mod sphere;
mod types;
mod utils;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    let mut log = Logger::new(io::stderr());

    let image_width = 800;
    let image_height = (image_width as f64 / ASPECT_RATIO) as usize;
    let mut renderer = Renderer::new(image_width, image_height)
        .with_samples_per_pixel(100)
        .with_max_depth(50);

    let world: [Box<dyn Hittable>; 2] = [
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    log.msg("Render frame ").flush();

    let timer = renderer.render(world.as_slice(), Fading::<2>::Ramp([0.1, 0.5]));
    log.elapsed(&timer).ln();

    log.msg("Output image ").flush();

    let timer = renderer.output();
    log.elapsed(&timer).ln();
}
