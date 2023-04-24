use cart::Cart;
use egui::FontDefinitions;
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
        window_resizable: true,
        // window_width: 640,
        // window_height: 360,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut cart = Cart::new();
    let grid = 0.15;
    let vingette = load_texture("vingette2.png").await.unwrap();

    #[derive(PartialEq)]
    enum Enum {
        First,
        Second,
        Third,
    }
    let mut my_f32: f32 = 0.;
    let mut my_string: String = "Hello World!".to_owned();
    let mut my_boolean: bool = true;
    let mut my_enum: Enum = Enum::First;

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

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
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

        if is_key_pressed(KeyCode::RightAlt) {
            break;
        }

        cart.display(
            // Color::new(0.50, 0.85, 1.00, 1.00),
            WHITE,
            0.006,
            6. * grid,
            3. * grid,
            0.3,
            0.12,
        );

        set_default_camera();
        draw_texture_ex(
            vingette,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .title_bar(false)
                .show(egui_ctx, |ui| {
                    ui.label("This is a label");
                    ui.hyperlink("https://github.com/emilk/egui");
                    ui.text_edit_singleline(&mut my_string);
                    if ui.button("Click me").clicked() {}
                    ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
                    ui.add(egui::DragValue::new(&mut my_f32));

                    ui.checkbox(&mut my_boolean, "Checkbox");

                    ui.horizontal(|ui| {
                        ui.radio_value(&mut my_enum, Enum::First, "First");
                        ui.radio_value(&mut my_enum, Enum::Second, "Second");
                        ui.radio_value(&mut my_enum, Enum::Third, "Third");
                    });

                    ui.separator();
                });
        });
        egui_macroquad::draw();
        next_frame().await;
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
