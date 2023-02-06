#[path = "./aaron.rs"]
mod aaron;

use nannou::prelude::*;
use aaron::*;

// struct Model<'a> {
//     _window: Option<Window>,
//     image_path: &'a str,
//     draw_callbacks: DrawCallbacks<'a>,
// }
// impl Model<'_> {
//     fn new() -> Self {
//         Self {
//             _window: None,
//             image_path: "",
//             draw_callbacks: DrawCallbacks::new(),
//         }
//     }
// }

static mut image_path: &str = "";

struct Nannou<'a> {
    image_path: &'a str,
}
impl<'a> Nannou<'a> {
    pub fn new() -> Self {
        Self {
            image_path: "",
        }
    }
    pub fn set_image_path(&mut self, path: &str) {
        // self.image_path = path;
        image_path = path;
    }
    pub fn run(&mut self) {
        nannou::sketch(Nannou::view)
        // .update(update)
        // .event(event)
        .run();
    }
    fn view(app: &App, frame: Frame) {
        // Prepare to draw.
        let mut draw = app.draw();
        let window = app.main_window();
        app.set_loop_mode(LoopMode::loop_once());
        // clear bg
        draw.background().color(WHITE);

        read_file(self.image_path, &mut draw_callbacks);

        draw.to_frame(app, &frame).unwrap();
    }
}