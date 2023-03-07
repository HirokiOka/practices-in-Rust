use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    let win = app.window_rect();

    let t = app.time * 0.02;

    for i in -10..10 {
        let x = (i * 20) as f32;
        let y = 10.0;
        draw.ellipse()
            .x_y(x, y)
            .radius(10.0)
            .hsv(t, 1.0, 1.0);
    }




    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
