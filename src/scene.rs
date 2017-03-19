use geom::sphere::Sphere;
use geom::RayIntersection;
use ray::Ray;

use cgmath::Vector3;

pub struct Scene {
    geom: Vec<Sphere>,        
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            geom: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.geom.push(sphere);
    }

}
impl<'a> Scene 
{
    pub fn intersections(&'a self, ray: &Ray<f64>) 
        -> Vec<(Vector3<f64>, &'a Sphere)> 
    {
        let mut intersections = Vec::new();

        for obj in &self.geom {
            let intersection = obj.ray_intersection(ray);
            if let Some(val) = intersection {
                intersections.push((val.intersection, obj));
            }
        }

        intersections
    }
}
