use crate::{
    scene::{Camera, Hit, Material, Scene, Sphere},
    window::Framebuffer,
};
use nalgebra::{vector, UnitVector3, Vector2, Vector3};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}

pub struct Raytracer<'a> {
    framebuffer: &'a mut Framebuffer,
}

impl<'a> Raytracer<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer) -> Self {
        Self { framebuffer }
    }

    pub fn draw_scene(&mut self, scene: &Scene) {
        self.draw(|pixel, (w, h)| {
            let pos = vector![pixel[0] as f32, pixel[1] as f32];
            let ray = Self::cast(*scene.camera(), pos, w, h);
            if let Some((hit, mat)) = Self::scene_intersect(scene, &ray) {
                let mut diffuse_intensity = 0.0;
                scene.lights().for_each(|light| {
                    let direction = (light.position - hit.p).normalize();
                    diffuse_intensity += light.intensity * direction.dot(&hit.normal).max(0.0);
                });

                return Some(mat.diffuse * diffuse_intensity.min(1.0));
            }

            None
        });
    }

    pub fn draw(&mut self, f: impl Fn(Vector2<usize>, (usize, usize)) -> Option<Vector3<f32>>) {
        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

        for y in 0..height {
            for x in 0..width {
                let pos = vector![x, y];
                if let Some(color) = f(pos, (width, height)) {
                    self.framebuffer.set_pixel(pos, color);
                }
            }
        }
    }

    /// Casts a ray from the camera into a pixel on the screen.
    fn cast(camera: Camera, pos: Vector2<f32>, w: usize, h: usize) -> Ray {
        let (w, h) = (w as f32, h as f32);
        let Camera { origin, fov } = camera;
        let aspect_ratio = w / h;

        // TODO(Bech): Forklar algoritme / credit.
        let x = (2.0 * (pos[0] + 0.5) / w - 1.0) * (fov / 2.0).tan() * aspect_ratio;
        let y = -(2.0 * (pos[1] + 0.5) / h - 1.0) * (fov / 2.0).tan();
        let z = -1.0;

        // It is common convention that the camera faces in the negative z-direction.
        let direction = vector![x, y, z].normalize();
        Ray { origin, direction }
    }

    fn scene_intersect(scene: &Scene, ray: &Ray) -> Option<(Hit, Material)> {
        scene
            .objects()
            .filter_map(|item| item.0.intersect(ray).map(|hit| (hit, item.1)))
            .reduce(|accum, item| if item.0.t < accum.0.t { item } else { accum })
    }
}
