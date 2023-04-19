use nannou::prelude::*;

struct State {
    pub x: f32,
    pub v: f32,
    pub w: f32,
    pub th: f32,
}

#[derive(Default)]
pub struct Ball {
    pub vdot: f32,
    pub wdot: f32,
    m: f32,
    M: f32,
    l: f32,
    g: f32,
    damp: f32,
    F: f32,
    radius: f32,
    color: Srgb<u8>,
}

impl Ball {
    pub fn new(color: Srgb<u8>) -> Self {
        let x = 0.;
        let v = 0.;
        let vdot = 0.;
        let th = 0.0;
        let w = 0.0;
        let wdot = 0.0;
        let m = 1.0;
        let M = 1.0;
        let l = 1.0;
        let g = 1.0;
        let damp = 1.0;
        let F = 1.0;
        let radius = 10.0;
        Ball {
            x,
            v,
            vdot,
            th,
            w,
            wdot,
            m,
            M,
            l,
            g,
            damp,
            F,
            radius,
            color,
        }
    }

    pub fn update(&mut self, app: &App, update: Update) {
        let dt = update.since_last.as_secs_f32();
        self.v += self.vdot * dt;
        self.w += self.wdot * dt;
        self.x += self.v * dt;
        self.th += self.w * dt;
    }

    pub fn process_state(&mut self) {
        let denom = self.M + self.m - self.m * self.th.cos() * self.th.cos();
        let term = self.m * self.l * self.w * self.w * self.th.sin() - self.damp * self.v + self.F;
        self.vdot = (-self.m * self.g * self.th.cos() * self.th.sin() + term) / denom;
        self.wdot =
            (self.m + self.M) * self.g * self.th.sin() - self.th.cos() * term / (self.l * denom);
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .x(self.x)
            .y(0.)
            .radius(self.radius)
            .color(self.color);
    }
}
