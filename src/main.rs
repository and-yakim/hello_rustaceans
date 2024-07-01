extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use std::time;

struct Molecule {
    sphere: SceneNode,
    velocity: Vector3<f32>,
    position: Vector3<f32>,
}

const MASS: f32 = 1.0;
const RADIUS: f32 = 0.1;
const EPSILON: f32 = 0.2;
const SIGMA: f32 = 0.1;
const N: usize = 2;

impl Molecule {
    fn new(window: &mut Window, position: Vector3<f32>, velocity: Vector3<f32>) -> Self {
        let mut sphere = window.add_sphere(RADIUS);
        sphere.set_color(0.0, 1.0, 0.0);
        sphere.append_translation(&Translation3::from(position));

        Molecule {
            sphere,
            velocity,
            position,
        }
    }

    fn update_position(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.sphere
            .set_local_translation(Translation3::from(self.position));
    }
}

fn lennard_jones_force(r: Vector3<f32>, epsilon: f32, sigma: f32) -> Vector3<f32> {
    let r_norm = r.norm();
    let lj_scalar = 24.0 * epsilon * (2.0 * (sigma / r_norm).powi(12) - (sigma / r_norm).powi(6))
        / r_norm.powi(2);
    r * lj_scalar
}

fn update_velocities(molecules: &mut [Molecule], i: usize, j: usize, dt: f32) {
    let r = molecules[j].position - molecules[i].position;
    let force = lennard_jones_force(r, EPSILON, SIGMA);

    let acceleration1 = force / MASS;
    let acceleration2 = -force / MASS;

    molecules[i].velocity += acceleration1 * dt;
    molecules[j].velocity += acceleration2 * dt;
}

fn main() {
    let mut window = Window::new("Molecules");
    window.set_light(Light::StickToCamera);

    let start_instant = time::Instant::now();
    let time_limit = 5;

    let dt = 0.01; // Time step

    let mut molecules = vec![
        Molecule::new(
            &mut window,
            Vector3::new(-0.3, 0.0, 0.0),
            Vector3::new(0.1, 0.0, 0.0),
        ),
        Molecule::new(
            &mut window,
            Vector3::new(0.3, 0.05, 0.1),
            Vector3::new(-0.2, 0.0, 0.0),
        ),
    ];

    while window.render() {
        if start_instant.elapsed().as_secs() > time_limit {
            break;
        };
        for i in 0..(N - 1) {
            for j in (i + 1)..N {
                update_velocities(&mut molecules, i, j, dt);
            }
            molecules[i].update_position(dt);
        }
        molecules[N - 1].update_position(dt);
    }
}
