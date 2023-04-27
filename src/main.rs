use crate::{theme::setup_theme, ui::Graph};
use cart::Cart;
use egui::{pos2, Color32};
use macroquad::prelude::*;
use ui::{draw_blue_grid, draw_speedometer, draw_ui, draw_vingette};
mod camera;
mod cart;
mod state;
mod theme;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cart".to_string(),
        fullscreen: true,
        // window_resizable: false,
        // window_width: 1280,
        // window_height: 720,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let grid = 0.15;
    let w_init = 1280.;
    let mut cart = Cart::default();
    let vingette = load_texture("vingette.png").await.unwrap();
    let font = load_ttf_font("Ubuntu-Regular.ttf").await.unwrap();
    setup_theme();
    let mut forceplt = Graph::new(
        &["Force", "Thrust"],
        pos2((0.5 - 2. * grid) * w_init, 0.),
        egui::vec2(1.5, 1.) * grid * w_init,
        None,
    );
    let mut forceplt1 = Graph::new(
        &["PID", "Integral", "Derivative", "Error"],
        pos2((0.5 + 0.5 * grid) * w_init, 0.),
        egui::vec2(1.5, 1.) * grid * w_init,
        Some([Color32::WHITE, Color32::LIGHT_GREEN, Color32::LIGHT_RED].to_vec()),
    );
    next_frame().await;

    loop {
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        clear_background(BLUE);
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if get_time() > 0. {
            cart.update(get_frame_time() as f64);
        }
        forceplt.update([cart.F].to_vec());
        forceplt1.update([cart.int, -cart.state.w, cart.error].to_vec());

        draw_blue_grid(grid, SKYBLUE, 0.001, 3, 0.003);

        cart.display(WHITE, 0.006, 6. * grid, 3. * grid);
        draw_speedometer(
            &format!(
                "Angular Velocity ({}) {:.2}",
                if cart.state.w.is_sign_negative() {
                    "-"
                } else {
                    "+"
                },
                cart.state.w.abs()
            ),
            vec2(0., screen_height() / screen_width() - 0.75 * grid),
            0.08,
            cart.state.w as f32,
            9.,
            0.8,
            font,
            14.,
            false,
        );
        draw_speedometer(
            &format!(
                "Cart Velocity ({}) {:.2}",
                if cart.state.v.is_sign_negative() {
                    "-"
                } else {
                    "+"
                },
                cart.state.v.abs()
            ),
            vec2(0., screen_height() / screen_width() - 1.75 * grid),
            0.08,
            cart.state.v as f32,
            20.,
            0.8,
            font,
            14.,
            true,
        );
        draw_ui(w_init, grid, &mut cart, &mut forceplt, &mut forceplt1);
        draw_vingette(vingette);
        next_frame().await;
    }
}
