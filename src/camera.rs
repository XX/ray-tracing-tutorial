use crate::types::{Point3, Vector3};

#[derive(Debug, Default)]
pub struct Camera {
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    viewport_u: Vector3,
    viewport_v: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    center: Point3,
    upper_left_pixel: Point3,
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

    pub fn with_viewport_size(
        mut self,
        viewport_height: f64,
        image_width: usize,
        image_height: usize,
    ) -> Self {
        // Calculate the actual aspect ratio.
        let aspect_ration = image_width as f64 / image_height as f64;

        self.viewport_height = viewport_height;
        self.viewport_width = viewport_height * aspect_ration;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        self.viewport_u = Vector3::new(self.viewport_width, 0.0, 0.0);
        self.viewport_v = Vector3::new(0.0, -self.viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = self.viewport_u / image_width as f64;
        self.pixel_delta_v = self.viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        self.upper_left_pixel = self.calc_upper_left_pixel_loc();

        self
    }

    pub fn pixel_center(&self, i: usize, j: usize, offset: Option<Vector3>) -> Point3 {
        let offset = offset.unwrap_or_default();
        self.upper_left_pixel
            + self.pixel_delta_u * (i as f64 + offset.x)
            + self.pixel_delta_v * (j as f64 + offset.y)
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    fn calc_upper_left_pixel_loc(&self) -> Point3 {
        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, self.focal_length)
            - self.viewport_u / 2.0
            - self.viewport_v / 2.0;
        viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.0
    }
}
