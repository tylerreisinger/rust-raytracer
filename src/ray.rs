use cgmath;
use cgmath::{Vector3};
use approx::ApproxEq;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray<T> {
    origin: Vector3<T>,
    direction: Vector3<T>,
}

impl<T> Ray<T> {
    pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn direction(&self) -> &Vector3<T> {
        &self.direction
    }
    pub fn origin(&self) -> &Vector3<T> {
        &self.origin
    }
}

impl<T: ApproxEq + cgmath::BaseFloat> ApproxEq for Ray<T> 
    where T::Epsilon: Copy
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon,
                   max_relative: Self::Epsilon) -> bool {
        self.origin.relative_eq(&other.origin, epsilon, max_relative)
            && self.direction.relative_eq(&other.direction, 
                    epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, 
               max_ulps: u32) -> bool {
        self.origin.ulps_eq(&other.origin, epsilon, max_ulps)
            && self.direction.ulps_eq(&other.direction, epsilon, max_ulps)
    }
}
