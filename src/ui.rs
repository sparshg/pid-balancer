use std::collections::VecDeque;

use egui::{
    epaint::Shadow,
    plot::{CoordinatesFormatter, Corner, HLine, Legend, Line, Plot, PlotBounds, PlotPoints},
    Align, Align2, Color32, Context, DragValue, Frame, Layout, Pos2, Slider, Vec2,
};
use macroquad::prelude::*;

use crate::cart::{self, Cart};

pub struct Graph {
    title: &'static [&'static str],
    pos: Pos2,
    size: Vec2,
    history: Vec<VecDeque<f32>>,
    hsize: usize,
    colors: Vec<Color32>,
}

impl Graph {
    pub fn new(
        title: &'static [&'static str],
        pos: Pos2,
        size: egui::Vec2,
        colors: Option<Vec<Color32>>,
    ) -> Self {
        if let Some(colors) = &colors {
            assert!(title.len() == colors.len() + 1);
        }
        Graph {
            title,
            pos,
            size,
            history: (0..title.len() - 1).map(|_| VecDeque::new()).collect(),
            hsize: 100,
            colors: colors
                .unwrap_or_else(|| (0..title.len() - 1).map(|_| Color32::WHITE).collect()),
        }
    }

    pub fn y(&mut self, y: f32) {
        self.pos.y = y;
    }

    pub fn update(&mut self, track: Vec<f64>) {
        assert!(track.len() == self.history.len());
        for (i, &v) in track.iter().enumerate() {
            self.history[i].push_back(v as f32);
            if self.history[i].len() > self.hsize {
                self.history[i].pop_front();
            }
        }
    }

    pub fn draw(&self, ctx: &Context, clamp: f64) {
        egui::Window::new(self.title[0])
            .frame(Frame {
                inner_margin: egui::Margin::same(0.),
                outer_margin: egui::Margin::same(0.),
                rounding: egui::Rounding::none(),
                fill: Color32::TRANSPARENT,
                shadow: Shadow::NONE,
                stroke: egui::Stroke::new(2., Color32::WHITE),
            })
            .current_pos(self.pos)
            .default_size(self.size)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                Plot::new("example")
                    .width(self.size.x)
                    .height(self.size.y)
                    .show_axes([false, false])
                    .show_background(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .show_x(false)
                    .show_y(false)
                    .coordinates_formatter(
                        Corner::LeftBottom,
                        CoordinatesFormatter::new(|&point, _| format!("y: {:.3}", point.y)),
                    )
                    .legend(Legend::default().position(egui::plot::Corner::RightBottom))
                    .show(ui, |plot_ui| {
                        plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                            [0., -clamp * 1.1],
                            [self.hsize as f64, clamp * 1.1],
                        ));
                        plot_ui.hline(HLine::new(0.).color(Color32::WHITE).width(1.));
                        for i in 0..self.history.len() {
                            plot_ui.line(
                                Line::new(
                                    self.history[i]
                                        .iter()
                                        .enumerate()
                                        .map(|(i, &y)| [i as f64, y as f64])
                                        .collect::<PlotPoints>(),
                                )
                                .width(2.)
                                .color(self.colors[i])
                                .name(self.title[i + 1]),
                            );
                        }
                    })
                    .response
            });
    }
}

pub fn draw_speedometer(
    label: &str,
    center: macroquad::math::Vec2,
    radius: f32,
    speed: f32,
    max_speed: f32,
    extent: f32,
    font: Font,
    fsize: f32,
    oneway: bool,
) {
    let angle = if oneway {
        0.5 * (1. + extent) - speed.abs() / max_speed * extent
    } else {
        0.5 * (1. - speed / max_speed * extent)
    } * std::f32::consts::PI;
    let x = center.x + 0.8 * radius * angle.cos();
    let y = center.y + 0.8 * radius * angle.sin();
    let sides = 20;

    for i in 0..sides {
        let t = i as f32 / sides as f32;
        let rx = ((t * extent + 0.5 - 0.5 * extent) * std::f32::consts::PI).cos();
        let ry = ((t * extent + 0.5 - 0.5 * extent) * std::f32::consts::PI).sin();

        let p0 = vec2(center.x + radius * rx, center.y + radius * ry);
        let p00 = vec2(center.x + 1.1 * radius * rx, center.y + 1.1 * radius * ry);

        let rx = (((i + 1) as f32 / sides as f32 * extent + 0.5 - 0.5 * extent)
            * std::f32::consts::PI)
            .cos();
        let ry = (((i + 1) as f32 / sides as f32 * extent + 0.5 - 0.5 * extent)
            * std::f32::consts::PI)
            .sin();

        let p1 = vec2(center.x + radius * rx, center.y + radius * ry);
        let p11 = vec2(center.x + 1.1 * radius * rx, center.y + 1.1 * radius * ry);
        draw_line(p00.x, p00.y, p11.x, p11.y, 0.006, WHITE);
        draw_line(
            p0.x,
            p0.y,
            p1.x,
            p1.y,
            0.004
                * if oneway {
                    1. - t
                } else {
                    3. * t * t - 3. * t + 1.
                },
            WHITE,
        );
    }
    push_camera_state();
    set_default_camera();
    let size = measure_text(label, None, fsize as u16, 1.);
    draw_text_ex(
        label,
        (0.5 + center.x) * screen_width() - size.width * 0.5,
        0.5 * (screen_height() - center.y * screen_width()) + size.offset_y + size.height,
        TextParams {
            font: font,
            font_size: fsize as u16 * 2,
            font_scale: 0.5,
            color: Color::new(1., 1., 1., 0.75),
            ..Default::default()
        },
    );
    pop_camera_state();
    let n = vec2(center.y - y, x - center.x);
    draw_triangle(
        vec2(center.x, center.y) + n * 0.08,
        vec2(center.x, center.y) - n * 0.08,
        vec2(x, y),
        WHITE,
    )
}
pub fn draw_ui(w: f32, grid: f32, cart: &mut Cart, forceplt: &mut Graph, forceplt1: &mut Graph) {
    egui_macroquad::ui(|ctx| {
        // ctx.set_debug_on_hover(true);
        ctx.set_pixels_per_point(screen_width() / w);
        forceplt.y(2.);
        forceplt1.y(2.);
        egui::Window::new("Controls")
            .anchor(Align2::RIGHT_TOP, egui::emath::vec2(0., 0.))
            .pivot(Align2::RIGHT_TOP)
            .default_width(1.25 * grid * w + 4.)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    ui.add(
                        Slider::new(&mut cart.pid.0, 0.0..=150.0)
                            .drag_value_speed(0.2)
                            .text("P"),
                    );
                    ui.add(
                        Slider::new(&mut cart.pid.1, 0.0..=100.0)
                            .drag_value_speed(0.1)
                            .text("I"),
                    );
                    ui.add(
                        Slider::new(&mut cart.pid.2, 0.0..=40.)
                            .drag_value_speed(0.04)
                            .text("D"),
                    );
                });
                ui.separator();
                ui.separator();
                ui.columns(2, |cols| {
                    cols[0].with_layout(Layout::top_down(Align::Max), |ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.M)
                                    .clamp_range(0.0..=100.)
                                    .speed(0.05),
                            );
                            ui.label("M_cart");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.ml)
                                    .clamp_range(0.0..=100.)
                                    .speed(0.05),
                            );
                            ui.label("M_rod");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.b1)
                                    .clamp_range(0.0..=0.5)
                                    .speed(0.0002)
                                    .custom_formatter(|x, _| format!("{:.3}", x)),
                            );
                            ui.label("Drag");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.l)
                                    .clamp_range(0.1..=10.)
                                    .speed(0.05),
                            );
                            ui.label("L_rod");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.Fclamp).speed(1.));
                            ui.label("F_clamp");
                        });
                    });
                    cols[1].with_layout(Layout::top_down(Align::Max), |ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.m)
                                    .clamp_range(0.0..=100.)
                                    .speed(0.05),
                            );
                            ui.label("M_bob");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.mw)
                                    .clamp_range(0.0..=100.)
                                    .speed(0.05),
                            );
                            ui.label("M_wheel");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.b2)
                                    .clamp_range(0.0..=0.5)
                                    .speed(0.0002)
                                    .custom_formatter(|x, _| format!("{:.3}", x)),
                            );
                            ui.label("Ang_Drag");
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                DragValue::new(&mut cart.R)
                                    .clamp_range(0.0..=1.)
                                    .speed(0.005),
                            );
                            ui.label("R_wheel");
                        });
                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut cart.Finp).speed(1.));
                            ui.label("Input Force");
                        });
                    });
                });
            });

        egui::Window::new("Physics")
            .anchor(Align2::LEFT_TOP, egui::emath::vec2(0., 0.))
            .default_width(1.25 * grid * w + 4.)
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
                    ui.add(
                        Slider::new(&mut cart.steps, 1..=100)
                            .logarithmic(true)
                            .text("Steps / Frame"),
                    );
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
                        egui::reset_button(ui, &mut cart.state);
                    })
                });
            });
        forceplt.draw(ctx, cart.Fclamp);
        forceplt1.draw(ctx, 9.);
    });
    egui_macroquad::draw();
}

pub fn draw_blue_grid(grid: f32, color: Color, thickness: f32, bold_every: i32, bold_thick: f32) {
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

pub fn draw_vingette(tex: Texture2D) {
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
