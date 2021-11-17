use nannou::prelude::*;

pub fn teal() -> Srgba<u8> {
    return srgba(114, 196, 208, 75 as u8);
}

pub fn seafoam() -> Srgba<u8> {
    return srgba(165, 214, 195, 75 as u8);
}

fn main() {
    nannou::sketch(view).run()
}


struct Dot {
    start_x: f32,
    start_y: f32,
    radius: f32,
    color: Srgba<u8>,
}


fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    // Draw a purple triangle in the top left half of the window.
    let win = app.window_rect();
    draw.tri()
        .points(win.bottom_left(), win.top_left(), win.top_right())
        .color(teal());

    // Draw an ellipse to follow the mouse.
    let t = app.time;
    let size = 100.0;
    let x = t.cos() * size;
    let y = t.sin() * size;

    draw.ellipse()
        .x_y(x, y)
        .color(seafoam());

    for i in 0..100 {

        let offset = 0.25 * i as f32 + random_range(-0.05, 0.05);

        draw.ellipse()
            .x_y(x * offset, y * offset)
            .radius(15.0 + i as f32)
            .color(seafoam());
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
