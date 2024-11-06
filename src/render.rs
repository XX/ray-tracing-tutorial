use crate::camera::Camera;
use crate::object::Hittable;
use crate::types::{random_unit_vector_on_sphere, Color, Point3, Ray, Vector3};
use crate::utils::Timer;

pub struct Renderer {
    image_width: usize,
    image_height: usize,
    camera: Camera,
    frame: Vec<Color>,
    samples_per_pixel: usize,
    max_depth: usize,
    cached_colors: Vec<(Ray, Color)>,
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
            samples_per_pixel: 5,
            max_depth: 10,
            cached_colors: Vec::new(),
        }
    }

    pub fn with_samples_per_pixel(mut self, samples_per_pixel: usize) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn render<T: Hittable + ?Sized>(&mut self, world: &T) -> Timer {
        if !self.frame.is_empty() {
            self.frame = Vec::with_capacity(self.image_width * self.image_height);
        }
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.cached_colors.clear();

        let mut timer = Timer::start();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::BLACK;

                if self.samples_per_pixel == 1 {
                    pixel_color += self.ray_color(self.get_ray(i, j), world);
                } else if self.samples_per_pixel == 5 {
                    for ray in self.get_rays(i, j) {
                        pixel_color += self.ray_color(ray, world);
                    }
                } else {
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_random_ray(i, j);
                        pixel_color += self.ray_color(ray, world);
                    }
                }

                self.frame.push(
                    (pixel_color * pixel_samples_scale)
                        .to_gamma_2_color()
                        .clamp(),
                );
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

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        Ray::new(
            self.camera.center(),
            self.camera.pixel_center(i, j, None) - self.camera.center(),
        )
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
            self.get_ray(i, j),
        ]
    }

    fn ray_color<T: Hittable + ?Sized>(&self, ray: Ray, world: &T) -> Color {
        self.ray_color_diffuse_random(ray, world)
    }

    fn ray_color_diffuse_random<T: Hittable + ?Sized>(&self, mut ray: Ray, world: &T) -> Color {
        let mut fading = 1.0;
        let mut bounds = 0;

        loop {
            if bounds > self.max_depth {
                break Color::BLACK;
            } else if let Some(hit) = world.hit(&ray, 0.001..f64::INFINITY) {
                let direction = hit.normal + random_unit_vector_on_sphere();
                ray = Ray::new(hit.point, direction);
                fading *= 0.5;
                bounds += 1;
            } else {
                break fading * Color::gradient_white_to_blue(ray.direction.y);
            }
        }
    }

    fn ray_color_normal<T: Hittable + ?Sized>(&self, ray: Ray, world: &T) -> Color {
        // Objects normal color
        if let Some(hit) = world.hit(&ray, 0.0..f64::INFINITY) {
            return 0.5 * (hit.normal + Color::WHITE);
        }

        // Background gradient
        Color::gradient_white_to_blue(ray.direction.y)
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
