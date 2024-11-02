use crate::camera::Camera;
use crate::object::Hittable;
use crate::types::{Color, Point3, Ray};
use crate::utils::Timer;

pub struct Renderer {
    image_width: usize,
    image_height: usize,
    camera: Camera,
    frame: Vec<Color>,
}

impl Renderer {
    pub fn new(image_width: usize, image_height: usize) -> Self {
        let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), 1.0).with_viewport_size(
            2.0,
            image_width,
            image_height,
        );

        Self {
            image_width,
            image_height,
            camera,
            frame: Vec::with_capacity(image_width * image_height),
        }
    }

    pub fn render<T: Hittable + ?Sized>(&mut self, world: &T) -> Timer {
        if !self.frame.is_empty() {
            self.frame = Vec::with_capacity(self.image_width * self.image_height);
        }

        let mut timer = Timer::start();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let ray_direction = self.camera.pixel_center(i, j) - self.camera.center();
                let ray = Ray::new(self.camera.center(), ray_direction);

                let pixel_color = self.ray_color(&ray, world);
                self.frame.push(pixel_color);
            }
        }

        timer.stop();
        timer
    }

    pub fn output(&self) -> Timer {
        let mut timer = Timer::start();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let color = self.frame[i + j * self.image_width].to_byte();
                println!("{} {} {}", color[0], color[1], color[2]);
            }
        }

        timer.stop();
        timer
    }

    fn ray_color<T: Hittable + ?Sized>(&self, ray: &Ray, world: &T) -> Color {
        if let Some(hit) = world.hit(ray, 0.0..f64::INFINITY) {
            return (0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0).to_vec())).into();
        }

        let a = 0.5 * (ray.direction.y + 1.0);
        ((1.0 - a) * Color::new(1.0, 1.0, 1.0).to_vec() + a * Color::new(0.5, 0.7, 1.0).to_vec())
            .into()
    }
}
