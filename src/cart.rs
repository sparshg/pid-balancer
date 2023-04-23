#![allow(non_snake_case)]

use std::f64::consts::PI;

use macroquad::{
    prelude::{Color, BLACK},
    shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle_lines},
};

use crate::state::State;
pub struct Cart {
    m: f64,
    M: f64,
    int: f64,
    l: f64,
    g: f64,
    pub F: f64,
    state: State,
    R: f64,
    r: f64,
    m1: f64,
    m2: f64,
    m3: f64,
    mw: f64,
    b1: f64,
    b2: f64,
}

impl Cart {
    pub fn new() -> Self {
        let (M, m, ml, mw) = (50., 10., 10., 10.);
        let l = 300.0;
        let g = 1000.;
        let F = 0.0;
        let int = 0.0;
        let state = State::from(0.0, 0.0, 0.0, std::f64::consts::PI + 0.4);
        let R = 30.0;
        let r = 20.0;
        let (b1, b2) = (10., 5.);
        let m1 = m + M + ml + 3. * mw;
        let m2 = m + ml / 3.;
        let m3 = m + ml / 2.;

        Cart {
            m,
            M,
            l,
            g,
            F,
            int,
            R,
            r,
            state,
            b1,
            b2,
            m1,
            m2,
            m3,
            mw,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let error = PI - self.state.th;
        self.int += error * dt;
        if self.F.abs() != 10000.0 {
            self.F = 1000. * (error * 250.0 - self.state.w * 50.0 + self.int * 200.);
        } else {
            self.int = 0.;
        }

        let steps = 1;
        let dt = dt / steps as f64;
        for _ in 0..steps {
            let k1 = self.process_state(self.state);
            let k2 = self.process_state(self.state.after(k1, dt * 0.5));
            let k3 = self.process_state(self.state.after(k2, dt * 0.5));
            let k4 = self.process_state(self.state.after(k3, dt));

            let k_avg = (
                (k1.0 + 2.0 * k2.0 + 2.0 * k3.0 + k4.0) / 6.0,
                (k1.1 + 2.0 * k2.1 + 2.0 * k3.1 + k4.1) / 6.0,
                (k1.2 + 2.0 * k2.2 + 2.0 * k3.2 + k4.2) / 6.0,
                (k1.3 + 2.0 * k2.3 + 2.0 * k3.3 + k4.3) / 6.0,
            );
            self.state.update(k_avg, dt);
        }
    }

    pub fn process_state(&self, state: State) -> (f64, f64, f64, f64) {
        let (_, v, w, th) = state.unpack();

        let (s, c) = (th.sin(), th.cos());
        let d = self.m2 * self.l * self.l * self.m1 - self.m3 * self.m3 * self.l * self.l * c * c;
        let f2 = -self.m3 * self.m3 * self.l * self.l * w * w * s * c
            + self.m3 * self.l * self.b1 * v * c
            - self.m1 * (self.m3 * self.g * self.l * s + self.b2 * w);
        let f4 = self.m2 * self.m3 * self.l * self.l * self.l * w * w * s
            - self.m2 * self.l * self.l * self.b1 * v
            + self.m3 * self.m3 * self.l * self.l * self.g * s * c
            + self.m3 * self.l * self.b2 * w * c;

        // returns (vdot, v, wdot, w)
        (
            (f4 + self.m2 * self.l * self.l * self.F) / d,
            v,
            (f2 - self.m3 * self.l * c * self.F) / d,
            w,
        )
    }

    pub fn display(&self, color: Color, thickness: f32, length: f32, depth: f32, w: f32, h: f32) {
        draw_line(-length, -depth, length, -depth, thickness, color);
        let ticks = 25;
        for i in 1..ticks - 2 {
            draw_line(
                -length + (2. / ticks as f32) * i as f32,
                -depth,
                -length + (2. / ticks as f32) * i as f32 - 0.03,
                -depth - 0.03,
                thickness,
                color,
            );
        }

        let scale = 0.001;
        let x = self.state.x as f32 * scale;
        let R = self.R as f32 * scale;
        let (c, s) = ((x / R).cos(), (x / R).sin());

        // cart
        draw_rectangle_lines(x - 0.5 * w, -depth + 2. * R, w, h, thickness * 2., color);

        // wheels
        draw_circle_lines(x - 0.30 * w, -depth + R, R, thickness, color);
        draw_circle_lines(x + 0.30 * w, -depth + R, R, thickness, color);
        draw_line(
            x - 0.30 * w,
            -depth + R,
            x - 0.30 * w - R * c,
            -depth + R + R * s,
            thickness,
            color,
        );
        draw_line(
            x + 0.30 * w,
            -depth + R,
            x + 0.30 * w - R * c,
            -depth + R + R * s,
            thickness,
            color,
        );

        let (c, s) = ((self.state.th).cos() as f32, (self.state.th).sin() as f32);
        let (l, r) = (self.l as f32 * scale, self.r as f32 * scale);
        // pendulum
        draw_line(
            x,
            -depth + h + 2. * R,
            x + (l - r) * s,
            -depth + h + 2. * R - (l - r) * c,
            thickness,
            color,
        );
        draw_circle_lines(x + l * s, -depth + h + 2. * R - l * c, r, thickness, color);
        draw_circle(x, -depth + 2. * R + h, 0.01, color);
    }
}
