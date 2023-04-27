#![allow(non_snake_case)]

use std::f64::consts::PI;

use macroquad::prelude::*;

use crate::state::State;
#[derive(PartialEq, Eq)]
pub enum Integrator {
    Euler,
    RungeKutta4,
}

impl Default for Integrator {
    fn default() -> Self {
        Self::RungeKutta4
    }
}

#[derive(PartialEq)]
pub struct Cart {
    pub F: f64,
    pub Fclamp: f64,
    pub Finp: f64,
    pub ui_scale: f32,
    pub enable: bool,
    pub pid: (f64, f64, f64),
    pub error: f64,
    pub int: f64,
    pub state: State,
    pub integrator: Integrator,
    pub steps: i32,
    pub m: f64,
    pub M: f64,
    pub mw: f64,
    pub ml: f64,
    pub l: f64,
    pub b1: f64,
    pub b2: f64,
    pub R: f64,
    g: f64,
    m1: f64,
    m2: f64,
    m3: f64,
}

impl Default for Cart {
    fn default() -> Self {
        let (M, m, ml, mw) = (5., 0.5, 1., 1.);
        let m1 = m + M + ml + 3. * mw;
        let m2 = m + ml / 3.;
        let m3 = m + ml / 2.;

        Cart {
            m,
            M,
            l: 1.,
            g: 9.80665,
            F: 0.,
            Fclamp: 400.,
            Finp: 20.,
            int: 0.,
            error: 0.,
            R: 0.1,
            state: State::default(),
            b1: 0.01,
            b2: 0.005,
            ui_scale: 0.3,
            mw,
            ml,
            m1,
            m2,
            m3,
            pid: (40., 8., 2.5),
            steps: 5,
            enable: true,
            integrator: Integrator::default(),
        }
    }
}

impl Cart {
    pub fn update(&mut self, dt: f64) {
        self.state.update_camera(dt);
        let steps = if dt > 0.02 {
            (60. * dt) as i32
        } else {
            self.steps
        };
        let dt = dt / steps as f64;
        for _ in 0..steps {
            self.error = PI - self.state.th;
            self.int += self.error * dt;
            self.F = 0.;
            if self.enable {
                self.F = (10.
                    * (self.error * self.pid.0 + self.int * self.pid.1
                        - self.state.w * self.pid.2))
                    .clamp(-self.Fclamp, self.Fclamp);
            }
            if is_key_down(KeyCode::Left) {
                self.F = -self.Finp;
                self.int = 0.
            } else if is_key_down(KeyCode::Right) {
                self.F = self.Finp;
                self.int = 0.
            }
            let k1 = self.process_state(self.state);
            if self.integrator == Integrator::Euler {
                self.state.update(k1, dt);
                continue;
            }
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

    pub fn get_potential_energy(&self) -> f64 {
        // with respect to ground
        -self.m3 * self.g * self.l * self.state.th.cos()
    }
    pub fn get_kinetic_energy(&self) -> f64 {
        0.5 * self.m1 * self.state.v * self.state.v
            + 0.5 * self.m2 * self.state.w * self.state.w * self.l * self.l
            + self.m3 * self.state.v * self.state.w * self.l * self.state.th.cos()
    }
    pub fn get_total_energy(&self) -> f64 {
        self.get_potential_energy() + self.get_kinetic_energy()
    }

    pub fn display(&self, color: Color, thickness: f32, length: f32, depth: f32) {
        draw_line(-length, -depth, length, -depth, thickness, color);
        let x = (self.state.x - self.state.camera.unwrap().y) as f32 * self.ui_scale;
        let R = self.R as f32 * self.ui_scale;
        let (c, s) = (
            (self.state.x / self.R).cos() as f32,
            (self.state.x / self.R).sin() as f32,
        );

        let ticks = (9. / self.ui_scale) as i32;
        let gap = 2. / ticks as f32;
        let offset = (self.state.camera.unwrap().y as f32 * self.ui_scale) % gap;
        for i in 0..ticks + 2 {
            draw_line(
                (-offset + gap * i as f32 - 1.) * length,
                -depth - 0.002,
                (-offset + gap * i as f32 - 1.) * length - 0.1 * self.ui_scale,
                -depth - 0.1 * self.ui_scale,
                thickness,
                color,
            );
        }
        draw_rectangle(
            -1.,
            -depth - 0.001,
            1. - length - 0.003,
            -0.11 * self.ui_scale,
            BLUE,
        );
        draw_rectangle(
            length + 0.003,
            -depth - 0.001,
            1. - length - 0.003,
            -0.11 * self.ui_scale,
            BLUE,
        );

        let (w, h) = (R * 10., R * 3.5);
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
        let l = self.l as f32 * self.ui_scale;
        // pendulum
        draw_line(
            x,
            -depth + h + 2. * R,
            x + (l - R) * s,
            -depth + h + 2. * R - (l - R) * c,
            thickness,
            color,
        );
        draw_circle_lines(x + l * s, -depth + h + 2. * R - l * c, R, thickness, color);
        draw_circle(x, -depth + 2. * R + h, 0.01, color);
    }
}
