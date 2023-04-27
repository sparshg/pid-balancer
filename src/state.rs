use std::f64::consts::PI;

use crate::camera::CameraDynamics;

#[derive(Clone, Copy, PartialEq)]

pub struct State {
    pub x: f64,
    pub v: f64,
    pub w: f64,
    pub th: f64,
    pub camera: Option<CameraDynamics>,
}

impl Default for State {
    fn default() -> Self {
        Self::from(0.0, 0.0, 0.0, PI + 0.5)
    }
}

impl State {
    pub fn from(x: f64, v: f64, w: f64, th: f64) -> Self {
        State {
            x,
            v,
            w,
            th,
            camera: Some(CameraDynamics::new(1.5, 0.75, 0., 0.0)),
        }
    }

    pub fn update(&mut self, (vdot, v, wdot, w): (f64, f64, f64, f64), dt: f64) {
        self.w += wdot * dt;
        self.th += w * dt;
        self.th = (self.th % (2. * PI) + 2. * PI) % (2. * PI);
        self.v += vdot * dt;
        self.x += v * dt;
    }

    pub fn update_camera(&mut self, dt: f64) {
        if let Some(camera) = &mut self.camera {
            camera.update(self.x, self.v, dt);
        }
    }

    pub fn after(&self, (vdot, v, wdot, w): (f64, f64, f64, f64), dt: f64) -> State {
        let mut new_state = self.clone();
        new_state.update((vdot, v, wdot, w), dt);
        new_state
    }

    pub fn unpack(&self) -> (f64, f64, f64, f64) {
        (self.x, self.v, self.w, self.th)
    }
}
