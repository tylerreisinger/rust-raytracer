use std::mem;
use std::fmt;
use cgmath::{Vector3, InnerSpace};
use geom::{Geometry, GeometryIntersection, RayIntersection};
use ray::Ray;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    origin: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Vector3<f64>, radius: f64) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Geometry for Sphere {
    fn origin(&self) -> &Vector3<f64> {
        &self.origin
    }
}
impl RayIntersection for Sphere {
    fn ray_intersection(&self, ray: &Ray<f64>) -> Option<GeometryIntersection> {
        let a = 1.0;
        let b = 2.0 * ray.direction().dot((ray.origin() - self.origin()));
        let c = (ray.origin() - self.origin).magnitude2() - self.radius() * self.radius();

        let radicand = b * b - 4.0 * a * c;

        // sqrt(x) returns nan for x < 0. In this case, we have no
        // intersection and should halt further computation.
        if radicand < 0.0 {
            None
        } else {
            let mut t0 = (-b + f64::sqrt(radicand)) / 2.0 * a;
            let mut t1 = (-b - f64::sqrt(radicand)) / 2.0 * a;

            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }

            let mut t = t0;

            if t < 0.0 {
                t = t1;
            }

            let pos = ray.origin() + t * ray.direction();

            Some(GeometryIntersection {
                time: t,
                intersection: pos,
            })
        }
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere({} @ {:?})", self.radius, self.origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camera::*;
    use cgmath::*;
    use geom::*;

    #[test]
    fn test_ray_intersect() {
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, 20.0), 5.0);
        let camera = Camera::new(CameraScreen::new(10, 10), (1.0, 1.0), 1.0);

        let test_ray = camera.ray_from_px(5, 5);
        assert_relative_eq!(Vector3::new(0.0, 0.0, 15.0),
                            sphere.ray_intersection(&test_ray).unwrap().intersection);

        let ray2 = camera.ray_from_px(4, 5);
        assert_relative_eq!(Vector3::new(-1.627219318, 0.0, 15.272193184),
                            sphere.ray_intersection(&ray2).unwrap().intersection,
                            epsilon = 1e-6);
        let ray3 = camera.ray_from_px(0, 9);
        assert!(sphere.ray_intersection(&ray3).is_none());
    }

}
