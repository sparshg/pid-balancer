use cart::Cart;
use macroquad::prelude::*;
mod cart;
mod state;
fn window_conf() -> Conf {
    Conf {
        window_title: "Cart".to_string(),
        fullscreen: true,
        window_resizable: true,
        // window_width: 600,
        // window_height: 600,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut cart = Cart::new();
    let grid = 0.15;
    let vingette = load_texture("vingette2.png").await.unwrap();

    loop {
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        clear_background(BLUE);
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        cart.update(get_frame_time() as f64);

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
