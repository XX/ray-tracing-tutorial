use crate::types::{Point3, Vector3};

#[derive(Debug, Default)]
pub struct Viewport {
    pub height: f64,
    pub width: f64,
    pub u: Vector3,
    pub v: Vector3,
}

#[derive(Debug, Default)]
pub struct Pixel {
    pub delta_u: Vector3,
    pub delta_v: Vector3,
}

#[derive(Debug)]
pub struct Camera {
    focal_length: f64,
    viewport: Viewport,
    pixel: Pixel,
    center: Point3,
    upper_left_pixel: Point3,

    // Vertical view angle (field of view)
    vertical_fov: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            focal_length: 1.0,
            viewport: Viewport::default(),
            pixel: Pixel::default(),
            center: Point3::new(0.0, 0.0, 0.0),
            upper_left_pixel: Point3::new(0.0, 0.0, 0.0),
            vertical_fov: 90.0,
        }
    }
}

impl Camera {
    pub fn new(center: Point3, focal_length: f64) -> Self {
        Self::default()
            .with_center(center)
            .with_focal_length(focal_length)
    }

    pub fn with_center(mut self, center: Point3) -> Self {
        self.center = center;
        self
    }

    pub fn with_focal_length(mut self, focal_length: f64) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn with_vertical_fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn with_viewport_size(mut self, image_width: usize, image_height: usize) -> Self {
        // Calculate the actual aspect ratio.
        let aspect_ration = image_width as f64 / image_height as f64;
        let theta = self.vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height * self.focal_length;
        self.viewport.height = viewport_height;
        self.viewport.width = viewport_height * aspect_ration;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        self.viewport.u = Vector3::new(self.viewport.width, 0.0, 0.0);
        self.viewport.v = Vector3::new(0.0, -self.viewport.height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel.delta_u = self.viewport.u / image_width as f64;
        self.pixel.delta_v = self.viewport.v / image_height as f64;

        // Calculate the location of the upper left pixel.
        self.upper_left_pixel = self.calc_upper_left_pixel_loc();

        self
    }

    pub fn pixel_center(&self, i: usize, j: usize, offset: Option<Vector3>) -> Point3 {
        let offset = offset.unwrap_or_default();
        self.upper_left_pixel
            + self.pixel.delta_u * (i as f64 + offset.x)
            + self.pixel.delta_v * (j as f64 + offset.y)
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    fn calc_upper_left_pixel_loc(&self) -> Point3 {
        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, self.focal_length)
            - self.viewport.u / 2.0
            - self.viewport.v / 2.0;
        viewport_upper_left + (self.pixel.delta_u + self.pixel.delta_v) / 2.0
    }
}
