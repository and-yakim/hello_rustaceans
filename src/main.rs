extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

struct Molecule {
    sphere: SceneNode,
    velocity: Vector3<f32>,
    position: Vector3<f32>,
}

const MASS: f32 = 1.0;
const RADIUS: f32 = 0.1;

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

fn update_velocities(m1: &mut Molecule, m2: &mut Molecule, dt: f32, epsilon: f32, sigma: f32) {
    let r = m2.position - m1.position;
    let force = lennard_jones_force(r, epsilon, sigma);

    let acceleration1 = force / MASS;
    let acceleration2 = -force / MASS;

    m1.velocity += acceleration1 * dt;
    m2.velocity += acceleration2 * dt;
}

fn main() {
    let mut window = Window::new("Kiss3d: Molecules");
    window.set_light(Light::StickToCamera);

    let dt = 0.01; // Time step

    // Create molecules with initial positions and velocities
    let mut molecules = vec![
        Molecule::new(
            &mut window,
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.1, 0.0, 0.0),
        ),
        Molecule::new(
            &mut window,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(-0.1, 0.0, 0.0),
        ),
        // Add more molecules as needed
    ];

    while window.render() {
        for molecule in &mut molecules {
            molecule.update_position(dt);

            // Here you can add the logic for interactions between molecules
            // e.g., calculate forces, update velocities, handle collisions, etc.
        }
    }
}
