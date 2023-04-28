use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq)]
pub struct CameraDynamics {
    pub y: f64,
    yv: f64,
    k1: f64,
    k2: f64,
    k3: f64,
}

impl Default for CameraDynamics {
    fn default() -> Self {
        Self::new(1.5, 0.75, 0., 0.)
    }
}

impl CameraDynamics {
    pub fn new(f: f64, z: f64, r: f64, init: f64) -> Self {
        CameraDynamics {
            y: init,
            yv: 0.,
            k1: z / (PI * f),
            k2: 0.25 / (PI * PI * f * f),
            k3: r * z * 0.5 / (PI * f),
        }
    }

    pub fn update(&mut self, x: f64, xv: f64, dt: f64) {
        let k2 = self
            .k2
            .max(self.k1 * dt)
            .max(0.5 * dt * dt + 0.5 * dt * self.k1);
        self.y += dt * self.yv;
        self.yv += dt * (x + self.k3 * xv - self.y - self.k1 * self.yv) / k2;
    }
}
