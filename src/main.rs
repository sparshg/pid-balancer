mod ball;
use ball::Ball;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
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
    for _ in 0..30 {
        model.ball.update(app, update);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DIMGRAY);
    model.ball.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}
