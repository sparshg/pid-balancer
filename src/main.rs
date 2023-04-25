use std::f64::consts::PI;

use cart::Cart;
use egui::{Align, Align2, Color32, Layout, Pos2};
use macroquad::prelude::*;

use crate::{theme::setup_theme, ui::Graph};
mod camera;
mod cart;
mod state;
mod theme;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cart".to_string(),
        // fullscreen: true,
        // window_resizable: false,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let grid = 0.15;
    let mut cart = Cart::new();
    let vingette = load_texture("vingette2.png").await.unwrap();
    setup_theme();
    next_frame().await;
    let w_init = screen_width();

    let mut forceplt = Graph::new(&["Force", "Thrust"], [-4., 3.5], grid, [3., 2.], None);
    let mut forceplt1 = Graph::new(
        &["PID", "Integral", "Derivative", "Error"],
        [1., 3.5],
        grid,
        [3., 2.],
        Some([Color32::WHITE, Color32::LIGHT_GREEN, Color32::LIGHT_RED].to_vec()),
    );

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

        cart.display(
            // Color::new(0.50, 0.85, 1.00, 1.00),
            WHITE,
            0.006,
            6. * grid,
            3. * grid,
            0.3,
            0.12,
        );
        draw_ui(w_init, &mut cart, &mut forceplt, &mut forceplt1);
        draw_vingette(vingette);
        next_frame().await;
    }

    fn draw_ui(w: f32, cart: &mut Cart, forceplt: &mut Graph, forceplt1: &mut Graph) {
        egui_macroquad::ui(|ctx| {
            ctx.set_pixels_per_point(screen_width() / w);
            forceplt.scale_pos(screen_width() / w);
            forceplt1.scale_pos(screen_width() / w);
            egui::Window::new("Controls")
                .anchor(Align2::RIGHT_TOP, egui::emath::vec2(-0., 0.))
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut cart.pid.0, 0.0..=100.0).text("P"));
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut cart.pid.1, 0.0..=100.0).text("I"));
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut cart.pid.2, 0.0..=100.0).text("D"));
                        });
                    });
                });
            forceplt.draw(ctx, cart.Fclamp);
            forceplt1.draw(ctx, 9.);
        });
        egui_macroquad::draw();
    }

    fn draw_vingette(tex: Texture2D) {
        set_default_camera();
        draw_texture_ex(
            tex,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }

    fn draw_blue_grid(grid: f32, color: Color, thickness: f32, bold_every: i32, bold_thick: f32) {
        draw_line(0., -1., 0., 1., bold_thick, color);
        draw_line(-1., 0., 1., 0., bold_thick, color);
        for i in 1..=(1. / grid as f32) as i32 {
            let thickness = if i % bold_every == 0 {
                bold_thick
            } else {
                thickness
            };
            draw_line(i as f32 * grid, -1., i as f32 * grid, 1., thickness, color);
            draw_line(
                -i as f32 * grid,
                -1.,
                -i as f32 * grid,
                1.,
                thickness,
                color,
            );
            draw_line(-1., i as f32 * grid, 1., i as f32 * grid, thickness, color);
            draw_line(
                -1.,
                -i as f32 * grid,
                1.,
                -i as f32 * grid,
                thickness,
                color,
            );
        }
    }
}
