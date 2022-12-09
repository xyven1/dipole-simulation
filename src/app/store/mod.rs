use crate::simulation::dipole::ChargeSimulation;
use crate::simulation::dipole::DipoleSimulation;
use crate::simulation::dipole::Simulatable;
use std::ops::Deref;

mod mouse;
use self::mouse::*;

mod camera;
use self::camera::*;

pub struct Store {
    pub state: StateWrapper,
}

impl Store {
    pub fn new() -> Store {
        Store {
            state: StateWrapper(State::new()),
        }
    }

    pub fn msg(&mut self, msg: &Msg) {
        self.state.msg(msg);
    }
}

pub struct State {
    clock: f32,
    camera: Camera,
    mouse: Mouse,
    pub(crate) simulation: Box<dyn Simulatable>,
    show_scenery: bool,
    time_scale: f32,
}

impl State {
    fn new() -> State {
        State {
            /// Time elapsed since the application started, in milliseconds
            clock: 0.,
            camera: Camera::new(),
            mouse: Mouse::default(),
            simulation: Box::new(DipoleSimulation::new(1., 1., 1., 1.)),
            // simulation: Box::new(ChargeSimulation::new()),
            show_scenery: true,
            time_scale: 1.,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// The current time in milliseconds
    pub fn clock(&self) -> f32 {
        self.clock
    }

    pub fn show_scenery(&self) -> bool {
        self.show_scenery
    }

    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::AdvanceClock(dt) => {
                self.clock += dt;
            }
            Msg::MouseDown(x, y) => {
                self.mouse.set_pressed(true);
                self.mouse.set_pos(*x, *y);
            }
            Msg::MouseUp => {
                self.mouse.set_pressed(false);
            }
            Msg::MouseMove(x, y) => {
                if !self.mouse.get_pressed() {
                    return;
                }

                let (old_x, old_y) = self.mouse.get_pos();

                let x_delta = old_x as i32 - x;
                let y_delta = y - old_y as i32;

                self.camera.orbit_left_right(x_delta as f32 / 50.0);
                self.camera.orbit_up_down(y_delta as f32 / 50.0);

                self.mouse.set_pos(*x, *y);
            }
            Msg::Zoom(zoom) => {
                self.camera.zoom(*zoom);
            }
            Msg::ShowScenery(show_scenery) => {
                self.show_scenery = *show_scenery;
            }
            Msg::TimeScale(time_scale) => {
                self.time_scale = *time_scale;
            }
            Msg::UpdateSimulation(dt) => {
                self.simulation.update(*dt as f64 * self.time_scale as f64);
            }
        }
    }
}

pub struct StateWrapper(State);

impl Deref for StateWrapper {
    type Target = State;

    fn deref(&self) -> &State {
        &self.0
    }
}

impl StateWrapper {
    pub fn msg(&mut self, msg: &Msg) {
        let _ = &self.0.msg(msg);
    }
}

pub enum Msg {
    AdvanceClock(f32),
    MouseDown(i32, i32),
    MouseUp,
    UpdateSimulation(f32),
    MouseMove(i32, i32),
    Zoom(f32),
    ShowScenery(bool),
    TimeScale(f32),
}
