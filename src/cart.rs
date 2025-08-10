#![allow(non_snake_case)]

use crate::{camera::CameraDynamics, state::State};
use aule::prelude::*;
use macroquad::prelude::*;
use ndarray::Array2;
use std::{f64::consts::PI, mem::swap, time::Duration};

#[derive(PartialEq, Eq)]
pub enum IntegratorKind {
    Euler,
    RungeKutta4,
}

#[derive(PartialEq)]
pub struct Cart {
    pub Fclamp: f64,
    pub Finp: f64,
    pub enable: bool,
    pub pid: PID,
    pub integrator: IntegratorKind,
    pub steps: i32,
    pub physics: CartPhysics,
    pub ui: CartUI,
}

#[derive(PartialEq)]
pub struct CartPhysics {
    pub F: f64,
    pub m1: f64,
    pub m2: f64,
    pub m3: f64,
    pub l: f64,
    pub b1: f64,
    pub b2: f64,
    pub g: f64,
    pub state: State,
}

#[derive(PartialEq)]
pub struct CartUI {
    pub ui_scale: f32,
    pub m: f64,
    pub M: f64,
    pub mw: f64,
    pub ml: f64,
    pub R: f64,
    pub camera: CameraDynamics,
}

impl Default for Cart {
    fn default() -> Self {
        let (M, m, ml, mw) = (5., 0.5, 1., 1.);
        let m1 = m + M + ml + 3. * mw;
        let m2 = m + ml / 3.;
        let m3 = m + ml / 2.;

        Cart {
            Fclamp: Self::MAX_FORCE,
            Finp: 20.,
            pid: PID::new(Self::KP, Self::KI, Self::KD),
            steps: Self::STEP_SIZE,
            enable: true,
            integrator: IntegratorKind::default(),
            physics: CartPhysics::new(m1, m2, m3),
            ui: CartUI::new(m, M, mw, ml),
        }
    }
}

impl Default for IntegratorKind {
    fn default() -> Self {
        Self::Euler
    }
}

impl Cart {
    const MAX_FORCE: f64 = 400.;
    const KP: f32 = 40.;
    const KI: f32 = 8.;
    const KD: f32 = 2.5;
    const STEP_SIZE: i32 = 5;

    fn update_force(&mut self, error: Signal) {
        if self.enable {
            let pid_output = self.pid.output(error).value as f64;
            self.physics.F = (10. * pid_output).clamp(-self.Fclamp, self.Fclamp);
        } else {
            self.physics.F = 0.;
        }
    }

    fn process_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.physics.F = -self.Finp;
            self.pid.clear_integral();
        } else if is_key_down(KeyCode::Right) {
            self.physics.F = self.Finp;
            self.pid.clear_integral();
        }
    }

    fn integrate(&mut self, dt: f64) {
        let old_state: Array2<f32> = self.physics.state.into();
        let dt = Duration::from_secs_f64(dt);

        self.physics.state = match self.integrator {
            IntegratorKind::Euler => Euler::integrate(old_state, dt, &self.physics),
            IntegratorKind::RungeKutta4 => RK4::integrate(old_state, dt, &self.physics),
        }
        .into();
        self.physics.state.th = (self.physics.state.th % (2. * PI) + 2. * PI) % (2. * PI);
    }

    pub fn update(&mut self, dt: f64) {
        self.ui
            .camera
            .update(self.physics.state.x, self.physics.state.v, dt);

        let steps = if dt > 0.02 {
            ((self.steps * 60) as f64 * dt) as i32
        } else {
            self.steps
        };

        let dt = dt / steps as f64;
        for _ in 0..steps {
            let error = PI - self.physics.state.th;
            let error = (error as f32, Duration::from_secs_f64(dt)).into();

            self.update_force(error);

            self.process_input();

            self.integrate(dt);
        }
    }
}

impl CartPhysics {
    pub fn new(m1: f64, m2: f64, m3: f64) -> Self {
        Self {
            F: 0.,
            m1,
            m2,
            m3,
            l: 1.,
            b1: 0.01,
            b2: 0.005,
            g: 9.80665,
            state: State::default(),
        }
    }

    #[inline(always)]
    fn compute_d(&self, th: f64) -> f64 {
        let c = th.cos();

        // d = (m2 * m1 * l^2) - (m3 * l * cos(th))^2;
        self.m2 * self.l * self.l * self.m1 - self.m3 * self.m3 * self.l * self.l * c * c
    }

    #[inline(always)]
    fn compute_f2(&self, th: f64, v: f64, w: f64) -> f64 {
        let (s, c) = (th.sin(), th.cos());

        // f2 = -(m3)^2 * l^2 * w^2 * sin(th) * cos(th)
        //      + m3 * l * b1 * v * cos(th)
        //      - m1 * (m3 * g * l * sin(th) + self.b2 * w);
        -self.m3 * self.m3 * self.l * self.l * w * w * s * c + self.m3 * self.l * self.b1 * v * c
            - self.m1 * (self.m3 * self.g * self.l * s + self.b2 * w)
    }

    #[inline(always)]
    fn compute_f4(&self, th: f64, v: f64, w: f64) -> f64 {
        let (s, c) = (th.sin(), th.cos());

        // f4 = m2 * m3 * l^3 * w^2 * sin(th)
        //      - m2 * l^2 * b1 * v
        //      + m3^2 * l^2 * g * sin(th) * cos(th)
        //      + m3 * l * b2 * w * cos(th);
        self.m2 * self.m3 * self.l * self.l * self.l * w * w * s
            - self.m2 * self.l * self.l * self.b1 * v
            + self.m3 * self.m3 * self.l * self.l * self.g * s * c
            + self.m3 * self.l * self.b2 * w * c
    }

    pub fn simulate(&self, State { v, w, th, .. }: State) -> State {
        let d = self.compute_d(th);
        let f2 = self.compute_f2(th, v, w);
        let f4 = self.compute_f4(th, v, w);

        let v_dot = (f4 + self.m2 * self.l * self.l * self.F) / d;
        let w_dot = (f2 - self.m3 * self.l * th.cos() * self.F) / d;

        (v_dot, v, w_dot, w).into()
    }

    pub fn get_potential_energy(&self) -> f64 {
        // with respect to ground
        -self.m3 * self.g * self.l * self.state.th.cos()
    }

    pub fn get_kinetic_energy(&self) -> f64 {
        // (m1 * v^2) / 2 + (m2 * (w * l)^2) / 2 + m3 * v * w * l * cos(th)
        0.5 * self.m1 * self.state.v * self.state.v
            + 0.5 * self.m2 * self.state.w * self.state.w * self.l * self.l
            + self.m3 * self.state.v * self.state.w * self.l * self.state.th.cos()
    }

    pub fn get_total_energy(&self) -> f64 {
        self.get_potential_energy() + self.get_kinetic_energy()
    }
}

impl StateEstimation for CartPhysics {
    fn estimate(&self, state: Array2<f32>) -> Array2<f32> {
        let state: State = state.into();

        let mut next_state = self.simulate(state);
        swap(&mut next_state.x, &mut next_state.v);
        swap(&mut next_state.th, &mut next_state.w);

        Array2::from(next_state)
    }
}

impl CartUI {
    pub fn new(m: f64, M: f64, mw: f64, ml: f64) -> Self {
        Self {
            ui_scale: 0.3,
            m,
            M,
            mw,
            ml,
            R: 0.1,
            camera: CameraDynamics::default(),
        }
    }

    pub fn display(
        &self,
        back_color: Color,
        color: Color,
        thickness: f32,
        length: f32,
        depth: f32,
        physics: &CartPhysics,
    ) {
        draw_line(-length, -depth, length, -depth, thickness, color);
        let x = (physics.state.x - self.camera.y) as f32 * self.ui_scale;
        let R = self.R as f32 * self.ui_scale;
        let (c, s) = (
            (physics.state.x / self.R).cos() as f32,
            (physics.state.x / self.R).sin() as f32,
        );

        let ticks = (9. / self.ui_scale) as i32;
        let gap = 2. / ticks as f32;
        let offset = (self.camera.y as f32 * self.ui_scale) % gap;
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
            back_color,
        );
        draw_rectangle(
            length + 0.003,
            -depth - 0.001,
            1. - length - 0.003,
            -0.11 * self.ui_scale,
            back_color,
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

        let (c, s) = (
            (physics.state.th).cos() as f32,
            (physics.state.th).sin() as f32,
        );
        let l = physics.l as f32 * self.ui_scale;
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
