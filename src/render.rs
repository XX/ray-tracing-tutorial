use crate::camera::Camera;
use crate::object::Hittable;
use crate::types::{Color, Point3, Ray, Vector3};
use crate::utils::Timer;

pub struct Renderer {
    image_width: usize,
    image_height: usize,
    camera: Camera,
    frame: Vec<Color>,
    samples_per_pixel: usize,
}

impl Renderer {
    pub fn new(image_width: usize, image_height: usize, samples_per_pixel: usize) -> Self {
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
            samples_per_pixel,
        }
    }

    pub fn render<T: Hittable + ?Sized>(&mut self, world: &T) -> Timer {
        if !self.frame.is_empty() {
            self.frame = Vec::with_capacity(self.image_width * self.image_height);
        }
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let mut timer = Timer::start();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::BLACK.to_vec();

                if self.samples_per_pixel == 5 {
                    for ray in self.get_rays(i, j) {
                        pixel_color += self.ray_color(&ray, world).to_vec();
                    }
                } else {
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_random_ray(i, j);
                        pixel_color += self.ray_color(&ray, world).to_vec();
                    }
                }

                self.frame.push((pixel_color * pixel_samples_scale).into());
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

    /// Construct a camera ray originating from the origin and directed at randomly sampled
    /// point around the pixel location i, j.
    pub fn get_random_ray(&self, i: usize, j: usize) -> Ray {
        let origin = self.camera.center();
        let direction =
            self.camera.pixel_center(i, j, Some(sample_square())) - self.camera.center();

        Ray::new(origin, direction)
    }

    pub fn get_rays(&self, i: usize, j: usize) -> [Ray; 5] {
        let origin = self.camera.center();

        [
            Ray::new(
                origin,
                self.camera
                    .pixel_center(i, j, Some(Point3::new(0.5, 0.5, 0.0)))
                    - origin,
            ),
            Ray::new(
                origin,
                self.camera
                    .pixel_center(i, j, Some(Point3::new(-0.5, 0.5, 0.0)))
                    - origin,
            ),
            Ray::new(
                origin,
                self.camera
                    .pixel_center(i, j, Some(Point3::new(0.5, -0.5, 0.0)))
                    - origin,
            ),
            Ray::new(
                origin,
                self.camera
                    .pixel_center(i, j, Some(Point3::new(-0.5, -0.5, 0.0)))
                    - origin,
            ),
            Ray::new(origin, self.camera.pixel_center(i, j, None) - origin),
        ]
    }

    fn ray_color<T: Hittable + ?Sized>(&self, ray: &Ray, world: &T) -> Color {
        if let Some(hit) = world.hit(ray, 0.0..f64::INFINITY) {
            return (0.5 * (hit.normal + Color::WHITE.to_vec())).into();
        }

        let a = 0.5 * (ray.direction.y + 1.0);
        ((1.0 - a) * Color::WHITE.to_vec() + a * Color::new(0.5, 0.7, 1.0).to_vec()).into()
    }
}

// Returns the vector to a random point in the [-.5,-.5] - [+.5,+.5] unit square.
fn sample_square() -> Vector3 {
    Vector3::new(
        rand::random::<f64>() - 0.5,
        rand::random::<f64>() - 0.5,
        0.0,
    )
}
