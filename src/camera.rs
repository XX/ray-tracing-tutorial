use crate::types::{Basis, Point3, Vector3};

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
    viewport: Viewport,
    pixel: Pixel,
    upper_left_pixel: Point3,

    // Vertical view angle (field of view)
    vertical_fov: f64,

    // Point camera is looking from
    lookfrom: Point3,

    // Point camera is looking at
    lookat: Point3,

    // Camera-relative "up" direction
    viewup: Vector3,

    basis: Basis,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            viewport: Viewport::default(),
            pixel: Pixel::default(),
            upper_left_pixel: Point3::new(0.0, 0.0, 0.0),
            vertical_fov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            viewup: Vector3::new(0.0, 1.0, 0.0),
            basis: Basis::default(),
        }
    }
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3) -> Self {
        Self::default().with_lookfrom(lookfrom).with_lookat(lookat)
    }

    pub fn with_vertical_fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn with_lookfrom(mut self, lookfrom: Point3) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn with_lookat(mut self, lookat: Point3) -> Self {
        self.lookat = lookat;
        self
    }

    pub fn with_viewup(mut self, viewup: Vector3) -> Self {
        self.viewup = viewup;
        self
    }

    pub fn with_viewport_size(mut self, image_width: usize, image_height: usize) -> Self {
        // Calculate the actual aspect ratio.
        let aspect_ration = image_width as f64 / image_height as f64;
        let theta = self.vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height * self.focal_length();
        self.viewport.height = viewport_height;
        self.viewport.width = viewport_height * aspect_ration;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.basis.w = (self.lookfrom - self.lookat).normalize();
        self.basis.u = self.viewup.cross(&self.basis.w).normalize();
        self.basis.v = self.basis.w.cross(&self.basis.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        self.viewport.u = self.viewport.width * self.basis.u;
        self.viewport.v = -self.viewport.height * self.basis.v;

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

    pub fn lookfrom(&self) -> Point3 {
        self.lookfrom
    }

    pub fn focal_length(&self) -> f64 {
        (self.lookfrom - self.lookat).magnitude()
    }

    fn calc_upper_left_pixel_loc(&self) -> Point3 {
        let viewport_upper_left = self.lookfrom
            - (self.focal_length() * self.basis.w)
            - self.viewport.u / 2.0
            - self.viewport.v / 2.0;
        viewport_upper_left + (self.pixel.delta_u + self.pixel.delta_v) / 2.0
    }
}
