use crate::state::State;
use nannou::prelude::*;

pub struct Ball {
    m: f32,
    M: f32,
    l: f32,
    g: f32,
    damp: f32,
    pub F: f32,
    state: State,
    radius: f32,
    color: Srgb<u8>,
}

impl Ball {
    pub fn new(color: Srgb<u8>) -> Self {
        let m = 0.2;
        let M = 1.0;
        let l = 50.0;
        let g = 0.8;
        let damp = 2.0;
        let F = 0.0;
        let state = State::from(0.0, 0.0, 0.0, PI * 0.99);
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
        // let (x, v, w, t) = state.unpack();
        // let th = t - PI;
        // let denom = self.M + self.m - self.m * th.cos() * th.cos();
        // let term = self.m * self.l * w * w * th.sin() - self.damp * v + self.F;

        // let vdot = (-self.m * self.g * th.cos() * th.sin() + term) / denom;
        // let wdot = (self.m + self.M) * self.g * th.sin() - th.cos() * term / (self.l * denom);
        // return (vdot, wdot);
        let (x, v, w, th) = state.unpack();
        let (M, m, ml, mw, mcr, mcb) = (50., 10., 10., 10., 0., 0.);
        let m1 = m + M + ml;
        let m2 = m + ml / 3.;
        let m3 = m + ml / 2.;
        let m4 = m2 + mcb;
        let (l, R, r, rl) = (100., 10., 9., 8.);
        let rg = rl * mcb / (mcb + mw);
        let g = 100.;
        let F = self.F;

        let (b1, b2) = (0.1, 0.5);
        let c1 = (mw * 0.5 * (R * R + 2. * rg * rg)
            + mcb * (rl - rg) * (rl - rg)
            + m4 * (R * R + rg * rg)
            + mcr * 0.5 * (R * R + r * r))
            / (R * R);
        let c2 = (mcr * r - 2. * m4 * rg) / R;

        let d = 2. * m2 * l * l * (c1 + c2 * (x / R).cos() + 0.5 * m1)
            - m3 * m3 * l * l * th.cos().powi(2);

        let f2 = -m3 * m3 * l * l * w * w * th.sin() * th.cos() + m3 * l * b1 * v * th.cos()
            - 2. * (c1 + c2 * (x / R).cos() + 0.5 * m1) * (m3 * g * l * th.sin() - b2 * w)
            - (m3 * l * c2 / R) * (v * v + R * g) * (x / R).sin() * th.cos();

        let f4 = m2 * m3 * l * l * l * w * w * th.sin() - m2 * l * l * b1 * v
            + m3 * m3 * l * l * g * th.sin() * th.cos()
            + m3 * l * b2 * w * th.cos()
            + (m2 * l * l * c2 / R) * (v * v + R * g) * (x / R).sin();

        ((f4 + m2 * l * l * F) / d, (f2 - m3 * l * th.cos() * F) / d)
    }

    pub fn display(&self, draw: &Draw) {
        // draw ground
        draw.rect().x_y(0.0, 0.0).w_h(640.0, 10.0).color(WHITE);
        let (x, th) = (self.state.x, self.state.th);
        // draw rectangle at x
        draw.rect().x_y(x, 15.0).w_h(40.0, 20.0).color(self.color);
        // draw line at angle th from x
        draw.line()
            .start(pt2(x, 15.0))
            .end(pt2(x + self.l * th.sin(), 15.0 - self.l * th.cos()))
            .weight(2.0)
            .color(WHITE);
        // draw ball at end of line
        draw.ellipse()
            .x_y(x + self.l * th.sin(), 15.0 - self.l * th.cos())
            .w_h(self.radius * 2.0, self.radius * 2.0)
            .color(self.color);
    }
}
