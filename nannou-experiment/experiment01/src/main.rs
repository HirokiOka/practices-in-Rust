use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(640, 480)
        .view(view)
        .build()
        .unwrap();

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(100.0, 100.0)
        .z_degrees(45.0)
        .color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}
