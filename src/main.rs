mod ball;
mod state;
use ball::Ball;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .view(view)
        .run();
}

struct Model {
    ball: Ball,
}

fn model(app: &App) -> Model {
    app.new_window().size(640, 480).build().unwrap();
    Model {
        ball: Ball::new(BLUE),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.ball.update(update);
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            KeyPressed(Key::Left) => model.ball.F = -10000.0,
            KeyPressed(Key::Right) => model.ball.F = 10000.0,
            KeyReleased(Key::Left) => model.ball.F = 0.0,
            KeyReleased(Key::Right) => model.ball.F = 0.0,
            _ => (),
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DIMGRAY);
    model.ball.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}
