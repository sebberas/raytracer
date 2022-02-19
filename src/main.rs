use nalgebra::vector;
use raytracer::*;
use scene::*;
use window::*;

mod raytracer;
mod scene;
mod window;

const WIDTH: usize = 1000;
const HEIGHT: usize = (WIDTH as f32 / (16.0 / 9.0)) as usize;

// Optimization techniques:
// - Multithreading
// - Sort-by-depth algorithm
// - Tagged-unions.

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT);

    let mut scene = Scene::new();

    let ivory = Material {
        diffuse: vector![0.4, 0.3, 0.1],
    };

    let rubber = Material {
        diffuse: vector![0.3, 0.1, 0.1],
    };

    let b = Material {
        diffuse: vector![0.1, 0.1, 0.3],
    };

    scene.add_object(
        Sphere {
            center: vector![-3.0, 0.0, -16.0],
            radius: 2.0,
        },
        ivory,
    );

    scene.add_object(
        Sphere {
            center: vector![-1.0, -1.5, -12.0],
            radius: 2.0,
        },
        rubber,
    );

    scene.add_object(
        Sphere {
            center: vector![1.5, -0.5, -18.0],
            radius: 3.0,
        },
        rubber,
    );

    scene.add_object(
        Sphere {
            center: vector![7.0, 5.0, -18.0],
            radius: 4.0,
        },
        b,
    );

    scene.add_light(Light {
        position: vector![-20.0, 20.0, 20.0],
        intensity: 1.5,
    });

    let mut raytracer = Raytracer::new(window.framebuffer_mut());
    raytracer.draw_scene(&scene);

    while window.is_open() {
        window.update_buffer()
    }
}
