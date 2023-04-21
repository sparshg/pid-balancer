#[derive(Clone, Copy)]
pub struct State {
    pub x: f32,
    pub v: f32,
    pub w: f32,
    pub th: f32,
}

impl State {
    pub fn new() -> Self {
        let x = 0.;
        let v = 0.;
        let th = 0.0;
        let w = 0.0;
        State { x, v, w, th }
    }

    pub fn update(&mut self, (vdot, wdot): (f32, f32), dt: f32) {
        self.w += wdot * dt;
        self.th += self.w * dt;
        self.v += vdot * dt;
        self.x += self.v * dt;
        // *self
    }

    pub fn after(&self, (vdot, wdot): (f32, f32), dt: f32) -> State {
        let mut new_state = self.clone();
        new_state.update((vdot, wdot), dt);
        new_state
    }

    pub fn unpack(&self) -> (f32, f32, f32, f32) {
        (self.x, self.v, self.w, self.th)
    }
}
