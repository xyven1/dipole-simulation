use nalgebra::Vector3;

pub enum Objects {
    Charge,
    Dipole,
}

pub trait Object {
    fn update(
        &mut self,
        d_pos: Vector3<f64>,
        d_vel: Vector3<f64>,
        d_orient: Vector3<f64>,
        d_ang_vel: Vector3<f64>,
    );
    fn get_type(&self) -> Objects;
    fn get_pos(&self) -> Vector3<f64>;
    fn get_orientation(&self) -> Vector3<f64>;
    fn get_offset(&self) -> f64;
}

pub(crate) struct Dipole {
    mass: f64,
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    orientation: Vector3<f64>,
    angular_velocity: Vector3<f64>,
    charge: f64,
    offset: f64,
    moment: f64,
}

impl Dipole {
    pub fn new(
        mass: f64,
        position: Vector3<f64>,
        velocity: Vector3<f64>,
        orientation: Vector3<f64>,
        angular_velocity: Vector3<f64>,
        charge: f64,
        offset: f64,
    ) -> Self {
        Self {
            mass,
            position,
            velocity,
            orientation,
            angular_velocity,
            charge,
            offset,
            moment: mass * offset * offset,
        }
    }
    fn force_torque(
        &self,
        r: Vector3<f64>,
        o: Vector3<f64>,
        sim: &DipoleSimulation,
        index: usize,
    ) -> (Vector3<f64>, Vector3<f64>) {
        let mut force = Vector3::new(0., 0., 0.);
        let mut torque = Vector3::new(0., 0., 0.);
        for (i, d) in sim.dipoles.iter().enumerate() {
            if i == index {
                continue;
            }
            let dst_positive = r + o * self.offset;
            let dst_negative = r - o * self.offset;

            let src_positive = d.position + d.orientation * d.offset;
            let src_negative = d.position - d.orientation * d.offset;

            let interaction = coulomb(src_negative, -d.charge, dst_negative, -self.charge);
            force += interaction;
            torque += (dst_negative - r).cross(&interaction);
            /* let mag_torque_r = (dst_negative - r).magnitude();
            web_sys::console::log_1(
                &format!(
                    "magnitude of the torque should equal displacement {}",
                    mag_torque_r
                )
                .into(),
            ); */
            let interaction = coulomb(src_positive, d.charge, dst_negative, -self.charge);
            force += interaction;
            torque += (dst_negative - r).cross(&interaction);
            let interaction = coulomb(src_negative, -d.charge, dst_positive, self.charge);
            force += interaction;
            torque += (dst_positive - r).cross(&interaction);
            let interaction = coulomb(src_positive, d.charge, dst_positive, self.charge);
            force += interaction;
            torque += (dst_positive - r).cross(&interaction);
        }
        (force, torque)
    }
}

impl Object for Dipole {
    fn update(
        &mut self,
        d_pos: Vector3<f64>,
        d_vel: Vector3<f64>,
        d_orient: Vector3<f64>,
        d_ang_vel: Vector3<f64>,
    ) {
        self.position += d_pos;
        self.velocity += d_vel;
        self.orientation = rotate(self.orientation, d_orient);
        self.angular_velocity += d_ang_vel;
    }
    fn get_type(&self) -> Objects {
        Objects::Dipole
    }
    fn get_pos(&self) -> Vector3<f64> {
        self.position
    }
    fn get_orientation(&self) -> Vector3<f64> {
        self.orientation
    }
    fn get_offset(&self) -> f64 {
        self.offset
    }
}

pub struct Charge {
    mass: f64,
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    charge: f64,
}

fn coulomb(source: Vector3<f64>, source_q: f64, dest: Vector3<f64>, dest_q: f64) -> Vector3<f64> {
    let r: Vector3<f64> = dest - source;
    let r_mag = r.magnitude();
    let r_hat = r / r_mag;

    K * source_q * dest_q * r_hat / (r_mag * r_mag)
}

impl Charge {
    // calculate force on Object based on position
    fn force(&self, r: Vector3<f64>, sim: &ChargeSimulation, index: usize) -> Vector3<f64> {
        let mut force = Vector3::new(0., 0., 0.);
        for (i, c) in sim.charges.iter().enumerate() {
            if i == index {
                continue;
            }
            force += coulomb(c.position, c.charge, r, self.charge);
        }
        force
    }
}

impl Object for Charge {
    fn update(
        &mut self,
        d_pos: Vector3<f64>,
        d_vel: Vector3<f64>,
        d_orient: Vector3<f64>,
        d_ang_vel: Vector3<f64>,
    ) {
        self.position += d_pos;
        self.velocity += d_vel;
    }
    fn get_type(&self) -> Objects {
        Objects::Charge
    }
    fn get_pos(&self) -> Vector3<f64> {
        self.position
    }
    fn get_orientation(&self) -> Vector3<f64> {
        Vector3::zeros()
    }
    fn get_offset(&self) -> f64 {
        0.
    }
}

pub trait Simulatable {
    fn update(&mut self, dt: f64);
    fn get_objects(&self) -> Vec<&dyn Object>;
}

pub struct ChargeSimulation {
    charges: Vec<Charge>,
}

impl ChargeSimulation {
    pub fn new() -> Self {
        Self {
            charges: vec![
                Charge {
                    mass: 5.,
                    position: Vector3::new(0., 0., 0.),
                    velocity: Vector3::new(-0.0002, 0., 0.),
                    charge: 0.01,
                },
                Charge {
                    mass: 5.,
                    position: Vector3::new(0., 10., 0.),
                    velocity: Vector3::new(0.0002, 0., 0.),
                    charge: -0.01,
                },
            ],
        }
    }
}

impl Simulatable for ChargeSimulation {
    fn update(&mut self, dt: f64) {
        // niave approach
        /* for charge in 0..self.charges.len() {
            let mut force = Vector3::zeros();
            for other in 0..self.charges.len() {
                if charge == other {
                    continue;
                }
                let charge = &self.charges[charge];
                let other = &self.charges[other];
                let r = charge.position - other.position;
                let r2 = r.norm_squared();
                let r3 = r2 * r.norm();
                force += r * (charge.charge * other.charge / r3);
            }
            let vel = self.charges[charge].velocity;
            self.charges[charge].update(vel * dt, force * dt);
        } */
        // runga kutta 4
        for index in 0..self.charges.len() {
            let charge = &self.charges[index];
            let k1v = charge.force(charge.position, self, index) / charge.mass;
            let k1r = charge.velocity;

            let k2v = charge.force(charge.position + k1r * dt / 2., self, index) / charge.mass;
            let k2r = charge.velocity + k1v * dt / 2.;

            let k3v = charge.force(charge.position + k2r * dt / 2., self, index) / charge.mass;
            let k3r = charge.velocity + k2v * dt / 2.;

            let k4v = charge.force(charge.position + k3r * dt, self, index) / charge.mass;
            let k4r = charge.velocity + k3v * dt;

            let d_vel = (k1v + 2. * k2v + 2. * k3v + k4v) * dt / 6.;
            let d_pos = (k1r + 2. * k2r + 2. * k3r + k4r) * dt / 6.;
            let charge = &mut self.charges[index];
            charge.update(d_pos, d_vel, Vector3::zeros(), Vector3::zeros());
        }
    }
    fn get_objects(&self) -> Vec<&dyn Object> {
        self.charges.iter().map(|c| c as &dyn Object).collect()
    }
}

pub struct DipoleSimulation {
    dipoles: Vec<Dipole>,
}

static K: f64 = 1.0;

impl DipoleSimulation {
    pub(crate) fn new(mass1: f64, mass2: f64, charge1: f64, charge2: f64) -> DipoleSimulation {
        DipoleSimulation {
            dipoles: vec![
                Dipole::new(
                    mass1,
                    Vector3::zeros(),
                    Vector3::zeros(),
                    Vector3::new(1., 0., 0.),
                    Vector3::zeros(),
                    charge1,
                    1.,
                ),
                Dipole::new(
                    mass2,
                    Vector3::new(10., 0., 0.),
                    Vector3::zeros(),
                    Vector3::new(0., 1., 0.),
                    Vector3::zeros(),
                    charge2,
                    1.,
                ),
            ],
        }
    }
}

fn rotate(orientation: Vector3<f64>, omega: Vector3<f64>) -> Vector3<f64> {
    let rotation = nalgebra::Rotation3::from_scaled_axis(omega);
    rotation * orientation
}

impl Simulatable for DipoleSimulation {
    fn update(&mut self, dt: f64) {
        for index in 0..self.dipoles.len() {
            let dipole = &self.dipoles[index];
            let (k1v, l1v) = dipole.force_torque(dipole.position, dipole.orientation, self, index);
            let k1v = k1v / dipole.mass;
            let l1v = l1v / dipole.moment;
            let k1r = dipole.velocity;
            let l1r = dipole.angular_velocity;

            let rotation = l1v * dt / 2.;
            let (k2v, l2v) = dipole.force_torque(
                dipole.position + k1r * dt / 2.,
                rotate(dipole.orientation, rotation),
                self,
                index,
            );
            let k2v = k2v / dipole.mass;
            let l2v = l2v / dipole.moment;
            let k2r = dipole.velocity + k1v * dt / 2.;
            let l2r = dipole.angular_velocity + l1v * dt / 2.;

            let rotation = l2v * dt / 2.;
            let (k3v, l3v) = dipole.force_torque(
                dipole.position + k2r * dt / 2.,
                rotate(dipole.orientation, rotation),
                self,
                index,
            );
            let k3v = k3v / dipole.mass;
            let l3v = l3v / dipole.moment;
            let k3r = dipole.velocity + k2v * dt / 2.;
            let l3r = dipole.angular_velocity + l2v * dt / 2.;

            let rotation = l3v * dt;
            let (k4v, l4v) = dipole.force_torque(
                dipole.position + k3r * dt,
                rotate(dipole.orientation, rotation),
                self,
                index,
            );
            let k4v = k4v / dipole.mass;
            let l4v = l4v / dipole.moment;
            let k4r = dipole.velocity + k3v * dt;
            let l4r = dipole.angular_velocity + l3v * dt;

            let d_vel = (k1v + 2. * k2v + 2. * k3v + k4v) * dt / 6.;
            let d_pos = (k1r + 2. * k2r + 2. * k3r + k4r) * dt / 6.;
            let d_ang_vel = (l1v + 2. * l2v + 2. * l3v + l4v) * dt / 6.;
            let d_orient = (l1r + 2. * l2r + 2. * l3r + l4r) * dt / 6.;
            let dipole = &mut self.dipoles[index];
            dipole.update(d_pos, d_vel, d_orient, d_ang_vel);
        }
    }

    fn get_objects(&self) -> Vec<&dyn Object> {
        self.dipoles.iter().map(|c| c as &dyn Object).collect()
    }
}
