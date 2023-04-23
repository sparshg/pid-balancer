use ball::Ball;
use macroquad::prelude::*;
mod ball;
mod state;
fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_string(),
        fullscreen: false,
        window_resizable: true,
        // window_width: 1400,
        // window_height: 800,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut ball = Ball::new();
    loop {
        clear_background(LIGHTGRAY);
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        ball.update(get_frame_time() as f64);
        ball.display();

        next_frame().await
    }
}
