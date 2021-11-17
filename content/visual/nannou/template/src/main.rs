use nannou::prelude::*;

use std::fs;
use std::io::ErrorKind;


const PROJ: &str = "template";

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

struct Model {
    main_window: WindowId,
    frames_dir: String,
    cur_frame: u32,
    recording: bool,
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // update logic here

    if model.recording && app.elapsed_frames() % 2 == 0 {
        model.cur_frame += 1;
        if model.cur_frame > 9999 {
            model.recording = false;
        } else {
            let filename = format!("{}/{}{:>04}.png",
                model.frames_dir,
                PROJ,
                model.cur_frame);
            match app.window(model.main_window) {
                Some(window) => {
                    window.capture_frame(filename);
                }
                None => {}
            }
        }
    }
}

fn random_color() -> Srgba<u8> {
    let random_opacity = random_range(50, 75) + 25;
    let teal = srgba(114, 196, 208, random_opacity as u8);
    let seafoam = srgba(165, 214, 195, random_opacity as u8);
    let grey = srgba(10, 10, 10, 100 as u8);
    let white = srgba(255, 255, 255, 100 as u8);

    let choice = random_range(0, 4);

    return match choice {
        0 => teal,
        1 => seafoam,
        2 => white,
        _ => grey,
    };

}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // draw something

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            if model.recording {
                model.recording = false;
            } else {
                fs::create_dir(&model.frames_dir).unwrap_or_else(|error| {
                    if error.kind() != ErrorKind::AlreadyExists {
                        panic! {"Problem creating directory {:?}", model.frames_dir};
                    }
                });
                model.recording = true;
                model.cur_frame = 0;
            }
        }
        Key::S => match app.window(model.main_window) {
            Some(window) => {
                window.capture_frame(app.exe_name().unwrap() + ".png");
            }
            None => {}
        },
        Key::Right => {
            // do something
        }
        Key::Left => {
            // do something
        }
        _other_key => {}
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait());
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let frames_dir = app.exe_name().unwrap() + "_frames";
    let recording = false;
    let cur_frame = 0;

    // build model

    let the_model = Model {
        main_window,
        frames_dir,
        cur_frame,
        recording,
    };

    the_model
}

fn main() {
    nannou::app(model).update(update).run();
}
