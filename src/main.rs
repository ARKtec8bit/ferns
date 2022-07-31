use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    // draw.background().color(PLUM);
    let width = app.window_rect().w() / 2.;
    let height = app.window_rect().h() / 2.;
    let mut x = 0.;
    let mut y = 0.;

    for _i in 0..500 {
        let next_x: f32;
        let next_y: f32;
        let r = random_f32();
        if r < 0.01 {
            next_y = 0.16 * y;
            next_x = 0.;
        } else if r < 0.86 {
            next_x = 0.85 * x + 0.04 * y;
            next_y = -0.04 * x + 0.85 * y + 1.6;
        } else if r < 0.93 {
            next_x = 0.2 * x + -0.26 * y;
            next_y = 0.23 * x + 0.22 * y + 1.6;
        } else {
            next_x = -0.15 * x + 0.28 * y;
            next_y = 0.26 * x + 0.24 * y + 0.44;
        }
        x = next_x;
        y = next_y;

        let px = map_range(x, -2.182, 2.6558, -width, width);
        let py = map_range(y, 0., 9.9983, height, -height);
        draw.ellipse()
            .color(STEELBLUE)
            .x_y(px, py)
            .w_h(1., 1.)
            .stroke_weight(0.5);
    }
    draw.to_frame(app, &frame).unwrap();
}
