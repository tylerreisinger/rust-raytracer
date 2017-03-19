use cgmath::Vector3;
use ray::Ray;

pub mod sphere;

#[derive(Clone, Debug, PartialEq)]
pub struct GeometryIntersection {
    pub time: f64,
    pub intersection: Vector3<f64>,
}

pub trait Geometry {
    fn origin(&self) -> &Vector3<f64>;
}

pub trait RayIntersection {
    fn ray_intersection(&self, ray: &Ray<f64>) 
        -> Option<GeometryIntersection>;
}
