use ray::Ray;
use cgmath::Vector3;
use cgmath::InnerSpace;

#[derive(Clone, Debug)]
pub struct CameraScreen {
    pub px_wide: usize,
    pub px_high: usize,
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub screen: CameraScreen,
    pub physical_width: f64,
    pub physical_height: f64,
    pub ray_source_dist: f64,
}

impl CameraScreen {
    pub fn new(px_wide: usize, px_high: usize) -> CameraScreen {
        CameraScreen {
            px_wide: px_wide,
            px_high: px_high,
        }
    }
}

impl Camera {
    pub fn new(screen: CameraScreen, physical_size: (f64, f64), ray_dist: f64) -> Camera {
        Camera {
            screen: screen,
            physical_width: physical_size.0,
            physical_height: physical_size.1,
            ray_source_dist: ray_dist,
        }
    }

    pub fn screen(&self) -> &CameraScreen {
        &self.screen
    }
    pub fn physical_dimensions(&self) -> (f64, f64) {
        (self.physical_width, self.physical_height)
    }

    pub fn ray_from_px(&self, pix_x: usize, pix_y: usize) -> Ray<f64> {
        let ray_origin = Vector3::new(0.0, 0.0, -self.ray_source_dist);
        let offset_x = -((self.screen.px_wide as f64) / 2.0) + (pix_x as f64);
        let offset_y = -((self.screen.px_high as f64) / 2.0) + (pix_y as f64);

        let dx = self.physical_width / (self.screen.px_wide as f64);
        let dy = self.physical_height / (self.screen.px_high as f64);

        let target_vec = Vector3::new(offset_x * dx, offset_y * dy, 0.0);
        let direction = target_vec - ray_origin;

        Ray::new(target_vec, direction.normalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::*;

    #[test]
    fn test_ray_from_px() {
        let screen = CameraScreen::new(10, 10);
        let camera = Camera::new(screen, (1.0, 1.0), 1.0);

        let center_ray = camera.ray_from_px(5, 5);
        assert_relative_eq!(*center_ray.direction(), Vector3::new(0.0, 0.0, 1.0));
        assert_relative_eq!(*center_ray.origin(), Vector3::new(0.0, 0.0, 0.0));
        let top_left_ray = camera.ray_from_px(0, 0);

        assert_relative_eq!(*top_left_ray.origin(), Vector3::new(-0.5, -0.5, 0.0));
        assert_relative_eq!(*top_left_ray.direction(),
                            Vector3::new(-0.5, -0.5, 1.0).normalize());

        let ray1 = camera.ray_from_px(8, 3);
        assert_relative_eq!(*ray1.origin(), Vector3::new(0.3, -0.2, 0.0));
        assert_relative_eq!(*ray1.direction(), Vector3::new(0.3, -0.2, 1.0).normalize());
    }
}
