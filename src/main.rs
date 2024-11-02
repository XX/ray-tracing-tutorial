use std::io;

use render::Renderer;

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
    let mut renderer = Renderer::new(image_width, image_height);

    let world: [Box<dyn Hittable>; 2] = [
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    log.msg("Render frame ").flush();

    let timer = renderer.render(world.as_slice());
    log.elapsed(&timer).ln();

    log.msg("Output image ").flush();

    let timer = renderer.output();
    log.elapsed(&timer).ln();
}
