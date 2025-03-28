use std::io;

use camera::Camera;
use material::Dielectric;

use crate::material::{Lambertian, Metal};
use crate::object::Hittable;
use crate::render::{Fading, Renderer};
use crate::sphere::Sphere;
use crate::types::{Color, Point3};
use crate::utils::Logger;

mod camera;
mod material;
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
    let camera = final_scene_camera(image_width, image_height);
    let mut renderer = Renderer::new(camera, image_width, image_height)
        .with_samples_per_pixel(100)
        .with_max_depth(50);

    let world = final_scene_world();

    log.msg("Render frame ").flush();

    let timer = renderer.render(world.as_slice(), Fading::<2>::Const(0.98));
    log.elapsed(&timer).ln();

    log.msg("Output image ").flush();

    let timer = renderer.output();
    log.elapsed(&timer).ln();
}

fn final_scene_camera(image_width: usize, image_height: usize) -> Camera {
    Camera::new(Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0))
        .with_defocus_angle(0.6)
        .with_focus_dist(10.0)
        .with_vertical_fov(20.0)
        .with_viewport_size(image_width, image_height)
}

fn final_scene_world() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let ground = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)),
    );
    world.push(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    world.push(Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5..=1.0);
                    let fuzz = rand::random_range(0.0..=0.5);
                    world.push(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                } else {
                    // glass
                    world.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }

    let sphere = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5));
    world.push(Box::new(sphere));

    let sphere = Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    );
    world.push(Box::new(sphere));

    let sphere = Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    );
    world.push(Box::new(sphere));

    world
}

fn simple_scene_camera(image_width: usize, image_height: usize) -> Camera {
    Camera::new(Point3::new(-2.0, 2.0, 1.0), Point3::new(0.0, 0.0, -1.0))
        .with_defocus_angle(10.0)
        .with_focus_dist(3.4)
        .with_vertical_fov(20.0)
        .with_viewport_size(image_width, image_height)
}

fn simple_scene_world() -> [Box<dyn Hittable>; 5] {
    let ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.8, 0.8, 0.0)),
    );
    let central = Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    );
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5));
    let left_bubble = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Dielectric::new(1.0 / 1.5),
    );
    let right = Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Color::new(0.8, 0.6, 0.2), 1.0),
    );

    [
        Box::new(ground),
        Box::new(central),
        Box::new(left),
        Box::new(left_bubble),
        Box::new(right),
    ]
}
