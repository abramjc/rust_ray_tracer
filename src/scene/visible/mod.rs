use crate::common::{Intersection, Ray, VertexFormat};

mod material;
mod mesh;
mod sphere;

pub trait Visible<T: VertexFormat> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>>;
}
