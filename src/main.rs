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

impl Molecule {
    fn new(window: &mut Window, position: Vector3<f32>, velocity: Vector3<f32>) -> Self {
        let mut sphere = window.add_sphere(0.1); // Radius of the sphere
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
