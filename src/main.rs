use cart::Cart;
use egui::Align2;
use macroquad::prelude::*;

use crate::theme::get_theme;
mod camera;
mod cart;
mod state;
mod theme;
fn window_conf() -> Conf {
    Conf {
        window_title: "Cart".to_string(),
        // fullscreen: true,
        window_resizable: false,
        window_width: 640,
        window_height: 400,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut cart = Cart::new();
    let grid = 0.15;
    let vingette = load_texture("vingette2.png").await.unwrap();

    egui_macroquad::cfg(|ctx| {
        get_theme(ctx);
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!(
                // "../../../../Library/Fonts/Product Sans Regular.ttf"
                // "../../../../Library/Fonts/FiraCode-Regular.ttf"
                "../Jost-Regular.ttf"
            )),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "my_font".to_owned());
        // ctx.set_fonts(fonts);
    });

    loop {
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        clear_background(BLUE);
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        // if get_time() > 0. {
        cart.update(get_frame_time() as f64);
        // }

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
        draw_ui(&mut cart.pid);
        draw_vingette(vingette);
        next_frame().await;
    }

    fn draw_ui(pid: &mut (f64, f64, f64)) {
        // spawn window aligned to right edge
        egui_macroquad::ui(|ctx| {
            ctx.set_pixels_per_point(screen_width() / 720.);
            egui::Window::new("egui ❤ macroquad")
                // .auto_sized()
                .anchor(Align2::RIGHT_TOP, egui::emath::vec2(-0., 0.))
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        ui.label("This is a label");
                        // ui.widt();
                        if ui.button("Click me").clicked() {}
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut pid.0, 0.0..=100.0).text("P"));
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut pid.1, 0.0..=100.0).text("I"));
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut pid.2, 0.0..=100.0).text("D"));
                        });
                    });
                });
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
