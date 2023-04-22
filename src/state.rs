use nannou::prelude::PI;

#[derive(Clone, Copy)]
pub struct State {
    pub x: f64,
    pub v: f64,
    pub w: f64,
    pub th: f64,
}

impl State {
    pub fn new() -> Self {
        let x = 0.;
        let v = 0.;
        let th = std::f64::consts::PI;
        let w = 0.0;
        State { x, v, w, th }
    }

    pub fn from(x: f64, v: f64, w: f64, th: f64) -> Self {
        State { x, v, w, th }
    }

    pub fn update(&mut self, (vdot, v, wdot, w): (f64, f64, f64, f64), dt: f64) {
        self.w += wdot * dt;
        self.th += w * dt;
        self.v += vdot * dt;
        self.x += v * dt;
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
