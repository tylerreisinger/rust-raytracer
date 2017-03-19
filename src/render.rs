use camera::Camera;
use scene::Scene;

pub struct Renderer {
    camera: Camera,
}

impl Renderer {
    pub fn new(camera: Camera) -> Renderer {
        Renderer {
            camera: camera,
        }
    }

    pub fn render_scene(&self, scene: &Scene) -> Vec<u8> {
        let (width, height) = self.camera.screen_dimensions();
        let mut out: Vec<u8> = vec![0; width*height];

        for y in 0..self.camera.screen.px_high {
            for x in 0..self.camera.screen.px_wide {
                let ray = self.camera.ray_from_px(x, y);
                let intersections = scene.intersections(&ray);
                for (_, _) in intersections {
                    out[x+y*width] = 255u8;
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camera::{Camera, CameraScreen};
    use scene::Scene;
    use geom::sphere::Sphere;
    use cgmath::Vector3;

    #[test]
    fn test_render() {
        let camera = Camera::new(CameraScreen::new(5, 5), (1.0, 1.0), 1.0);
        let mut scene = Scene::new();
        scene.add_sphere(
            Sphere::new(Vector3::new(0.0, 0.0, 10.0), 3.0));
        let renderer = Renderer::new(camera);
        let pixels = renderer.render_scene(&scene);

        println!("{:?}", pixels);
        assert!(false);
    }
}
