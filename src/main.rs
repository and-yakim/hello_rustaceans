extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use std::time;

struct Molecule {
    sphere: SceneNode,
    velocity: Vector3<f32>,
    position: Vector3<f32>,
}

// const MASS: f32 = 1.0;
const RADIUS: f32 = 0.1;
const EPSILON: f32 = 0.1;
const SIGMA: f32 = 0.1;
const DT: f32 = 0.01;
const BOUNDRY_RADIUS: f32 = 1.0;
const MIN_TEMP: f32 = 0.0;
const MAX_TEMP: f32 = 0.1;

impl Molecule {
    fn new(window: &mut Window, position: Vector3<f32>, velocity: Vector3<f32>) -> Self {
        let mut sphere = window.add_sphere(RADIUS);
        sphere.append_translation(&Translation3::from(position));

        Molecule {
            sphere,
            velocity,
            position,
        }
    }

    fn update_state(&mut self) {
        self.update_velocity_by_boundry();
        self.update_position();
        self.set_temperature();
    }

    fn update_velocity_by_boundry(&mut self) {
        let pos_norm = self.position.norm();
        let pos_dir = self.position.normalize();
        let r = pos_dir * (BOUNDRY_RADIUS - pos_norm);
        let force = lennard_jones_force(r, EPSILON, SIGMA);
        self.velocity += force * DT;
    }

    fn update_position(&mut self) {
        self.position += self.velocity * DT;
        self.sphere
            .set_local_translation(Translation3::from(self.position));
    }

    fn set_temperature(&mut self) {
        let (r, g, b) = temperature_to_color(self.velocity.norm_squared(), MIN_TEMP, MAX_TEMP);
        self.sphere.set_color(r, g, b);
    }
}

fn temperature_to_color(temperature: f32, min_temp: f32, max_temp: f32) -> (f32, f32, f32) {
    let normalized_temp = (temperature - min_temp) / (max_temp - min_temp);
    let normalized_temp = normalized_temp.clamp(0.0, 1.0);

    let r = normalized_temp;
    let g = 1.0 - (normalized_temp - 0.5).abs() * 2.0;
    let b = 1.0 - normalized_temp;

    (r, g, b)
}

fn lennard_jones_force(r: Vector3<f32>, epsilon: f32, sigma: f32) -> Vector3<f32> {
    let r_norm = r.norm();
    let mupltiplier = (sigma / r_norm).powi(6);
    let lj_scalar = 24.0 * epsilon * (2.0 * mupltiplier.powi(2) - mupltiplier) / r_norm.powi(2);
    r * lj_scalar
}

fn update_velocities(molecules: &mut [Molecule], i: usize, j: usize) {
    let r = molecules[j].position - molecules[i].position;
    let force = lennard_jones_force(r, EPSILON, SIGMA);

    molecules[i].velocity += force * DT;
    molecules[j].velocity += -force * DT;
}

fn main() {
    let mut window = Window::new("Molecules");
    window.set_light(Light::StickToCamera);

    let start_instant = time::Instant::now();
    let time_limit = 15;

    let n: usize = 6;
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
        Molecule::new(
            &mut window,
            Vector3::new(0.6, -0.1, 0.0),
            Vector3::new(-0.3, 0.0, 0.0),
        ),
        Molecule::new(
            &mut window,
            Vector3::new(-0.8, 0.0, 0.0),
            Vector3::new(0.1, 0.0, 0.0),
        ),
        Molecule::new(
            &mut window,
            Vector3::new(0.8, 0.05, 0.1),
            Vector3::new(-0.2, 0.0, 0.0),
        ),
        Molecule::new(
            &mut window,
            Vector3::new(0.2, -0.1, 0.0),
            Vector3::new(-0.3, 0.0, 0.0),
        ),
    ];

    let eye = Point3::new(BOUNDRY_RADIUS * 2.0, 0.0, 0.0);
    let at = Point3::origin();

    while window.render_with_camera(&mut kiss3d::camera::ArcBall::new(eye, at)) {
        if start_instant.elapsed().as_secs() > time_limit {
            break;
        };
        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                update_velocities(&mut molecules, i, j);
            }
            molecules[i].update_state();
        }
        molecules[n - 1].update_state();
    }
}
