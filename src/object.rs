use std::ops::Range;

use crate::types::{Hit, Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit>;
}
