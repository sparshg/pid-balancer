use crate::{theme::setup_theme, ui::Graph};
use cart::Cart;
use egui::{epaint::Shadow, Align, Align2, Color32, DragValue, Frame, Layout, Slider};
use macroquad::prelude::*;
use ui::{draw_blue_grid, draw_speedometer, draw_vingette};
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
    let mut cart = Cart::default();
    let vingette = load_texture("vingette2.png").await.unwrap();
    let font = load_ttf_font("Ubuntu-Regular.ttf").await.unwrap();
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

        cart.display(WHITE, 0.006, 6. * grid, 3. * grid);
        draw_speedometer(
            "Angular Velocity",
            vec2(0., screen_height() / screen_width() - 0.75 * grid),
            0.08,
            cart.state.w as f32,
            9.,
            0.8,
            font,
            12.,
            false,
        );
        draw_speedometer(
            &format!(
                "Cart Velocity ({})",
                if cart.state.v.is_sign_negative() {
                    "-"
                } else {
                    "+"
                }
            ),
            vec2(0., screen_height() / screen_width() - 1.75 * grid),
            0.08,
            cart.state.v as f32,
            cart.Fclamp as f32 * 0.5,
            0.8,
            font,
            12.,
            true,
        );
        draw_ui(w_init, 3.5, grid, &mut cart, &mut forceplt, &mut forceplt1);
        draw_vingette(vingette);
        next_frame().await;
    }
}

fn draw_ui(
    w: f32,
    y_top: f32,
    grid: f32,
    cart: &mut Cart,
    forceplt: &mut Graph,
    forceplt1: &mut Graph,
) {
    egui_macroquad::ui(|ctx| {
        // ctx.set_debug_on_hover(true);
        ctx.set_pixels_per_point(screen_width() / w);
        forceplt.y(2.);
        forceplt1.y(2.);
        egui::Window::new("Controls")
            .anchor(Align2::RIGHT_TOP, egui::emath::vec2(0., 0.))
            .pivot(Align2::RIGHT_TOP)
            .default_width(1.25 * grid * screen_width() + 4.)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    ui.add(Slider::new(&mut cart.pid.0, 0.0..=150.0).text("P"));
                    ui.add(Slider::new(&mut cart.pid.1, 0.0..=100.0).text("I"));
                    ui.add(Slider::new(&mut cart.pid.2, 0.0..=40.0).text("D"));
                });
                ui.separator();
                ui.separator();
                ui.columns(2, |cols| {
                    cols[0].with_layout(Layout::top_down(Align::Max), |ui| {
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.M));
                            ui.label("M_cart");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.ml));
                            ui.label("M_rod");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.b1)
                                    .custom_formatter(|x, _| format!("{:.3}", x)),
                            );
                            ui.label("Drag");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.l));
                            ui.label("L_rod");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.Fclamp));
                            ui.label("F_clamp");
                        });
                    });
                    cols[1].with_layout(Layout::top_down(Align::Max), |ui| {
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.m));
                            ui.label("M_bob");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.mw));
                            ui.label("M_wheel");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.b2)
                                    .custom_formatter(|x, _| format!("{:.3}", x)),
                            );
                            ui.label("Ang_Drag");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.R));
                            ui.label("R_wheel");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.Finp));
                            ui.label("Input Force");
                        });
                    });
                });
            });

        egui::Window::new("Physics")
            .anchor(Align2::LEFT_TOP, egui::emath::vec2(0., 0.))
            .default_width(1.25 * grid * screen_width() + 4.)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            // .title_bar(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label(format!("System Energy: {:.2}", cart.get_total_energy()));
                    ui.label(format!("Kinetic Energy: {:.2}", cart.get_kinetic_energy()));
                    ui.label(format!(
                        "Potential Energy: {:.2}",
                        cart.get_potential_energy()
                    ));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Integrator: ");
                        ui.selectable_value(&mut cart.integrator, cart::Integrator::Euler, "Euler");
                        ui.selectable_value(
                            &mut cart.integrator,
                            cart::Integrator::RungeKutta4,
                            "Runge-Kutta⁴",
                        );
                    });
                    ui.separator();
                    ui.add(Slider::new(&mut cart.steps, 1..=100).text("Steps / Frame"));
                    ui.add(
                        Slider::new(&mut cart.ui_scale, 0.03..=0.6)
                            .custom_formatter(|n, _| format!("{:.2}", n / 0.3))
                            .custom_parser(|s| s.parse::<f64>().map(|v| v * 0.3).ok())
                            .text("Draw Scale"),
                    );
                    ui.separator();
                    ui.horizontal(|ui| {
                        let enable = cart.enable;
                        ui.label("System Controls: ");
                        ui.toggle_value(
                            &mut cart.enable,
                            if enable {
                                "Controller: ON"
                            } else {
                                "Controller: OFF"
                            },
                        );
                        egui::reset_button(ui, cart);
                    })
                });
            });
        forceplt.draw(ctx, cart.Fclamp);
        forceplt1.draw(ctx, 9.);
    });
    egui_macroquad::draw();
}
