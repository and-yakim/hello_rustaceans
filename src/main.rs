extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use rand::prelude::*;
use std::f32::consts::PI;
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
const MAX_VEL: f32 = 0.3;
const MAX_TEMP: f32 = MAX_VEL * MAX_VEL;
const N: usize = 20;

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
    if r.norm() <= 2.5 * RADIUS {
        let force = lennard_jones_force(r, EPSILON, SIGMA);

        molecules[i].velocity += force * DT;
        molecules[j].velocity += -force * DT;
    }
}

fn random_vector3_in_sphere(boundary_radius: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    // Generate a random radius r within the sphere
    let r = rng.gen_range(0.0..boundary_radius).cbrt() * boundary_radius;

    // Generate random angles theta and phi
    let theta = rng.gen_range(0.0..2.0 * PI);
    let phi = rng.gen_range(0.0..PI);

    // Convert spherical coordinates to Cartesian coordinates
    let x = r * phi.sin() * theta.cos();
    let y = r * phi.sin() * theta.sin();
    let z = r * phi.cos();

    Vector3::new(x, y, z)
}

fn random_position_with_check(molecules: &[Molecule]) -> Vector3<f32> {
    let try_ = random_vector3_in_sphere(BOUNDRY_RADIUS - RADIUS);
    if molecules.into_iter().fold(false, |acc, mol| {
        let res = (mol.position - try_).norm() < 2.5 * RADIUS;
        acc && res
    }) {
        random_position_with_check(molecules)
    } else {
        try_
    }
}

fn main() {
    let mut window = Window::new("Molecules");
    window.set_light(Light::StickToCamera);

    let start_instant = time::Instant::now();
    let time_limit = 15;

    let mut molecules: Vec<Molecule> = Vec::with_capacity(N);
    for _ in 0..N {
        let position = random_position_with_check(&molecules);
        let velocity = random_vector3_in_sphere(MAX_VEL);
        molecules.push(Molecule::new(&mut window, position, velocity));
    }

    let eye = Point3::new(BOUNDRY_RADIUS * 2.0, 0.0, 0.0);
    let at = Point3::origin();

    while window.render_with_camera(&mut kiss3d::camera::ArcBall::new(eye, at)) {
        if start_instant.elapsed().as_secs() > time_limit {
            break;
        };
        for i in 0..(N - 1) {
            for j in (i + 1)..N {
                update_velocities(&mut molecules, i, j);
            }
            molecules[i].update_state();
        }
        molecules[N - 1].update_state();
    }
}
