use ndarray::Array2;
use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct State {
    pub x: f64,
    pub v: f64,
    pub th: f64,
    pub w: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            x: 0.0,
            v: 0.0,
            th: PI + 0.5, // Initial angle offset
            w: 0.0,
        }
    }
}

impl From<(f64, f64, f64, f64)> for State {
    fn from(tuple: (f64, f64, f64, f64)) -> Self {
        Self {
            x: tuple.0,
            v: tuple.1,
            th: tuple.2,
            w: tuple.3,
        }
    }
}

impl State {
    pub fn next_state(mut self, state: State, dt: f64) -> State {
        self.v += state.x * dt;
        self.x += state.v * dt;

        self.w += state.th * dt;
        self.th += state.w * dt;
        self.th = (self.th % (2. * PI) + 2. * PI) % (2. * PI);
        self
    }
}

impl From<Array2<f32>> for State {
    fn from(array: Array2<f32>) -> Self {
        Self {
            x: array[[0, 0]] as f64,
            v: array[[1, 0]] as f64,
            th: array[[2, 0]] as f64,
            w: array[[3, 0]] as f64,
        }
    }
}

impl From<State> for Array2<f32> {
    fn from(state: State) -> Self {
        Array2::from_shape_vec(
            (4, 1),
            vec![
                state.x as f32,
                state.v as f32,
                state.th as f32,
                state.w as f32,
            ],
        )
        .unwrap()
    }
}
