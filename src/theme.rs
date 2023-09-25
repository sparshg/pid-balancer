use egui::{epaint, style, Color32, Context, Rounding};
use egui_macroquad::egui;

pub fn setup_theme() {
    egui_macroquad::cfg(|ctx| {
        get_theme(ctx);
    });
}

fn get_theme(ctx: &Context) {
    let bg = Color32::from_black_alpha(30);
    let act = style::WidgetVisuals {
        bg_fill: bg,
        bg_stroke: epaint::Stroke {
            width: 0.5,
            color: Color32::WHITE,
        },
        fg_stroke: epaint::Stroke {
            width: 0.5,
            color: Color32::WHITE,
        },
        weak_bg_fill: Color32::TRANSPARENT,
        rounding: Rounding::from(2.),
        expansion: 0.0,
    };
    let ina = style::WidgetVisuals {
        bg_fill: Color32::TRANSPARENT,
        bg_stroke: epaint::Stroke::NONE,
        fg_stroke: epaint::Stroke::NONE,
        weak_bg_fill: Color32::TRANSPARENT,
        rounding: Rounding::none(),

        expansion: 0.0,
    };
    ctx.set_visuals(egui::Visuals {
        dark_mode: false,
        window_shadow: epaint::Shadow::NONE,
        panel_fill: Color32::TRANSPARENT,
        window_fill: Color32::from_black_alpha(50),
        override_text_color: Some(Color32::WHITE),
        window_stroke: epaint::Stroke::NONE,
        faint_bg_color: Color32::TRANSPARENT,
        extreme_bg_color: bg,
        widgets: style::Widgets {
            noninteractive: ina,
            inactive: act,
            hovered: act,
            active: act,
            open: act,
        },
        ..Default::default()
    });
    // ctx.set_visuals(style::Visuals::dark());
}
