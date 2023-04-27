use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();


    let points = (0..50).map(|i| {
      let x = i as f32 - 25.0;          //subtract 25 to center the sine wave
      let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
      (point, STEELBLUE)
    });
    draw.polyline()
        .weight(3.0)
        .points_colored(points);



    draw.to_frame(app, &frame).unwrap();
}
