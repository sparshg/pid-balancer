use crate::state::State;
use nannou::prelude::*;

pub struct Ball {
    m: f32,
    M: f32,
    l: f32,
    g: f32,
    damp: f32,
    F: f32,
    state: State,
    radius: f32,
    color: Srgb<u8>,
}

impl Ball {
    pub fn new(color: Srgb<u8>) -> Self {
        let m = 1.0;
        let M = 1.0;
        let l = 1.0;
        let g = 1.0;
        let damp = 1.0;
        let F = 1.0;
        let state = State::new();
        let radius = 10.0;
        Ball {
            m,
            M,
            l,
            g,
            damp,
            F,
            radius,
            color,
            state,
        }
    }

    pub fn update(&mut self, update: Update) {
        let dt = update.since_last.as_secs_f32();

        let k1 = self.process_state(self.state);
        let k2 = self.process_state(self.state.after(k1, dt * 0.5));
        let k3 = self.process_state(self.state.after(k2, dt * 0.5));
        let k4 = self.process_state(self.state.after(k3, dt));

        let (vdot, wdot) = (
            (k1.0 + 2.0 * k2.0 + 2.0 * k3.0 + k4.0) / 6.0,
            (k1.1 + 2.0 * k2.1 + 2.0 * k3.1 + k4.1) / 6.0,
        );
        self.state.update((vdot, wdot), dt);
    }

    pub fn process_state(&self, state: State) -> (f32, f32) {
        let (_, v, w, th) = state.unpack();

        let denom = self.M + self.m - self.m * th.cos() * th.cos();
        let term = self.m * self.l * w * w * th.sin() - self.damp * v + self.F;

        let vdot = (-self.m * self.g * th.cos() * th.sin() + term) / denom;
        let wdot = (self.m + self.M) * self.g * th.sin() - th.cos() * term / (self.l * denom);

        (vdot, wdot)
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .x(0.)
            .y(0.)
            .radius(self.radius)
            .color(self.color);
    }
}
