use std::collections::VecDeque;

use egui::{
    epaint::Shadow,
    plot::{CoordinatesFormatter, Corner, HLine, Legend, Line, Plot, PlotBounds, PlotPoints},
    style, vec2, Align, Align2, Color32, Context, Frame, Pos2, Vec2,
};
use macroquad::window::{screen_height, screen_width};

pub struct Graph {
    title: &'static [&'static str],
    pos: Pos2,
    grid_pos: [f32; 2],
    size: Vec2,
    grid: f32,
    history: Vec<VecDeque<f32>>,
    hsize: usize,
    colors: Vec<Color32>,
}

impl Graph {
    pub fn new(
        title: &'static [&'static str],
        pos: [f32; 2],
        grid: f32,
        size: [f32; 2],
        colors: Option<Vec<Color32>>,
    ) -> Self {
        if let Some(colors) = &colors {
            assert!(title.len() == colors.len() + 1);
        }
        Graph {
            title: title,
            grid_pos: pos,
            pos: Pos2 {
                x: (0.5 + 0.5 * pos[0] * grid) * screen_width(),
                y: 0.5 * screen_height() - 0.5 * pos[1] * grid * screen_width(),
            },
            size: egui::vec2(
                0.5 * size[0] * grid * screen_width(),
                0.5 * size[1] * grid * screen_width(),
            ),
            grid,
            history: (0..title.len() - 1).map(|_| VecDeque::new()).collect(),
            hsize: 100,
            colors: colors
                .unwrap_or_else(|| (0..title.len() - 1).map(|_| Color32::WHITE).collect()),
        }
    }

    pub fn scale_pos(&mut self, pixels_per_point: f32) {
        self.pos = Pos2 {
            x: self.pos.x,
            y: (0.5 * screen_height() - 0.5 * self.grid_pos[1] * self.grid * screen_width())
                / pixels_per_point,
        };
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
                    .coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default())
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
