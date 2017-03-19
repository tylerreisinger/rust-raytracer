extern crate num;
extern crate cgmath;
#[macro_use]
extern crate approx;
extern crate image;

pub mod camera;
pub mod ray;
pub mod geom;
pub mod render;
pub mod scene;

use std::path::Path;

use camera::{Camera, CameraScreen};
use geom::sphere::Sphere;
use scene::Scene;
use cgmath::Vector3;
use render::Renderer;

fn main() {
    let camera = Camera::new(CameraScreen::new(500, 500), (1.0, 1.0), 1.0);
    let mut scene = Scene::new();
    scene.add_sphere(
        Sphere::new(Vector3::new(0.0, 0.0, 10.0), 3.0));
    scene.add_sphere(
        Sphere::new(Vector3::new(1.2, 0.6, 6.5), 1.0));

    let img_path = Path::new("./test_out.png");

    let (width, height) = camera.screen_dimensions();
    let renderer = Renderer::new(camera);
    let pixels = renderer.render_scene(&scene);

    image::save_buffer(img_path, &*pixels, 
        width as u32, height as u32,
        image::Gray(8)).unwrap();
}
