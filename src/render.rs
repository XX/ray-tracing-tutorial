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
    max_depth: usize,
}

impl Renderer {
    pub fn new(image_width: usize, image_height: usize) -> Self {
        let camera = Camera::new(Point3::new(-2.0, 2.0, 1.0), Point3::new(0.0, 0.0, -1.0))
            .with_defocus_angle(10.0)
            .with_focus_dist(3.4)
            .with_vertical_fov(20.0)
            .with_viewport_size(image_width, image_height);

        Self {
            image_width,
            image_height,
            camera,
            frame: Vec::with_capacity(image_width * image_height),
            samples_per_pixel: 5,
            max_depth: 10,
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

    pub fn render<const FADING_N: usize, T: Hittable + ?Sized>(
        &mut self,
        world: &T,
        fading: Fading<FADING_N>,
    ) -> Timer {
        if !self.frame.is_empty() {
            self.frame = Vec::with_capacity(self.image_width * self.image_height);
        }
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let mut timer = Timer::start();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::BLACK;
                let fading = match fading {
                    Fading::Const(fading) => fading,
                    Fading::Ramp(fadings) => fadings[i * fadings.len() / self.image_width],
                };

                if self.samples_per_pixel == 1 {
                    pixel_color += self.ray_color(self.get_ray(i, j), world, fading);
                } else if self.samples_per_pixel == 5 {
                    for ray in self.get_rays(i, j) {
                        pixel_color += self.ray_color(ray, world, fading);
                    }
                } else {
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_random_ray(i, j);
                        pixel_color += self.ray_color(ray, world, fading);
                    }
                }

                self.frame.push(
                    (pixel_samples_scale * pixel_color)
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
        let origin = self.camera.origin();
        let direction = self.camera.pixel_center(i, j, Some(sample_square())) - origin;

        Ray::new(origin, direction)
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let origin = self.camera.origin();
        Ray::new(origin, self.camera.pixel_center(i, j, None) - origin)
    }

    pub fn get_rays(&self, i: usize, j: usize) -> [Ray; 5] {
        let origin = self.camera.origin();

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

    fn ray_color<T: Hittable + ?Sized>(&self, ray: Ray, world: &T, fading: f64) -> Color {
        self.ray_color_diffuse_random(ray, world, fading)
    }

    fn ray_color_diffuse_random<T: Hittable + ?Sized>(
        &self,
        mut ray: Ray,
        world: &T,
        fading: f64,
    ) -> Color {
        let mut attenuation = Color::WHITE;
        let mut acc_fading = 1.0;
        let mut bounds = 0;

        loop {
            if bounds > self.max_depth {
                break Color::BLACK;
            } else if let Some(hit) = world.hit(&ray, 0.001..f64::INFINITY) {
                if let Some((scattered, new_attenuation)) = hit.material.scatter(&ray, &hit) {
                    ray = scattered;
                    attenuation *= new_attenuation;
                    acc_fading *= fading;
                    bounds += 1;
                } else {
                    break Color::BLACK;
                }
            } else {
                break attenuation * (acc_fading * Color::gradient_white_to_blue(ray.direction.y));
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

pub enum Fading<const N: usize = 2> {
    Const(f64),
    Ramp([f64; N]),
}
