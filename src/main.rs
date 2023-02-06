use std::{fs::File, io, thread, time};

// mod nannou_draw;

// use aaron::{DrawCallbacks, read_file};
// use valora::prelude::*;
use nannou::prelude::*;
use std::io::Read;
// use raylib::prelude::*;

fn main() {
    // nannou_draw::draw_image("d:/misc/bevy/src/aa0");
    // nannou::app(model_handler)
    //     .event(event_handler)
    //     .simple_window(view)
    //     .run();
}

struct Model {
    _window: window::Id,
    _img: String,
}

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// fn model_handler(_app: &App) -> Model {
    // let window = app.new_window()
    //     .size(WIDTH, HEIGHT)
    //     .view(view)
    //     .build()
    //     .unwrap();
    // let img = file_to_text("d:/misc/bevy/src/aa0").unwrap();
    // return Model { _window: window, _img: img };
// }

fn event_handler(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
}

//utility functions
fn file_to_text(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_text = String::new();
    file.read_to_string(&mut file_text)?;

    return Ok(file_text);
}