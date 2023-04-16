use nannou::prelude::*;

#[derive(Default)]
pub struct Ball {
    pub pos: Point2,
    pub vel: Vec2,
    pub acc: Vec2,
    radius: f32,
    color: Srgb<u8>,
}

impl Ball {
    pub fn new(color: Srgb<u8>) -> Self {
        let pos = pt2(0.0, 0.0);
        let vel = vec2(0.0, 0.0);
        let acc = vec2(0.0, -10000.0);
        let radius = 10.0;
        Ball {
            pos,
            vel,
            acc,
            radius,
            color,
        }
    }

    pub fn update(&mut self, app: &App, update: Update) {
        let dt = update.since_last.as_secs_f32();

        // Runge-Katta 4th order
        let rk_mult = dt + dt * dt / 2.0 + dt * dt * dt / 6.0 + dt * dt * dt * dt / 24.0;

        self.vel += self.acc * rk_mult;
        self.pos += self.vel * rk_mult;

        if self.pos.y - self.radius < -240.0 {
            self.vel.y *= -1.0;
            self.pos.y = -240.0 + self.radius;
        }
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.pos)
            .radius(self.radius)
            .color(self.color);
    }
}
