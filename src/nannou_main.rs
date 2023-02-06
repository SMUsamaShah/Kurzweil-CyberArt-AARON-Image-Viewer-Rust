mod nannou_draw;

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
// use std::str;
use std::vec;
// use nannou::color;
// use seek_bufread::BufReader;
// use nannou::ui::color::{self, rgb};

use nannou::{prelude::*};

// brush props
struct Brush {
    px: f32,
    py: f32,
    x: f32,
    y: f32,
    size: usize,
    color: Vec<f32>,
    v_px_py: Vector2,
    v_x_y: Vector2,
}
impl Brush {
    pub fn new() -> Self {
        Self {
            px: 0.0, py: 0.0, 
            x: 0.0, y: 0.0, 
            size: 1, 
            color: vec![0.0 as f32; 3],
            v_px_py: Vector2::new(0.0, 0.0),
            v_x_y: Vector2::new(0.0, 0.0),
        }
    }
    pub fn px_py(&mut self, px : f32, py : f32) {
        self.px = px;
        self.py = py;
        self.v_px_py[0] = px;
        self.v_px_py[1] = py;
    }
    pub fn x_y(&mut self, x : f32, y : f32) {
        self.x = x;
        self.y = y;
        self.v_x_y[0] = x;
        self.v_x_y[1] = y;
    }
    pub fn dx_dy(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
        self.v_x_y[0] = self.x;
        self.v_x_y[1] = self.y;
    }
}

fn main() {
    // nannou::sketch(view).run();
    nannou::app(model).update(update).run();
}
struct Model {
    _window: window::Id,
    img: String,
}
fn model(app: &App) -> Model {
    let _window = app.new_window().size(1920, 1080).view(view).build().unwrap();
    let img = read_image("d:/aaron_images/aa0").unwrap();
    Model { _window, img }
}
fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let mut draw = app.draw();
    let window = app.main_window();
    app.set_loop_mode(LoopMode::loop_once());

    // // Clear the background to purple.
    // draw.background().color(PLUM);

    // // Draw a blue ellipse with default size and position.
    // draw.ellipse().color(STEELBLUE);

    
    draw = draw.x_y(-(window.rect().w() as f32)*0.5, -(window.rect().h() as f32)*0.5);
                //.scale(0.5);
    
    let _ = draw_image(&draw, &window, &model, &app, &frame);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn read_image(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_text = String::new();
    file.read_to_string(&mut file_text)?;

    return Ok(file_text);
    // Ok(())
}

fn draw_image(draw: &Draw, window: &Window, model: &Model, app: &App, frame: &Frame) -> io::Result<()> {
    let reader = &model.img;//BufReader::new(file);
    let img_lines = reader.lines().collect::<Vec<_>>();

    let mut i = 0;
    let mut res_x: usize = 0;
    let mut res_y: usize = 0;
    let mut num_colors: usize = 0;
    let mut color_pallet = vec![vec![0.0 as f32; 3]; 0];
    let mut colors_filled = false;
    // let mut draw = app.draw();
    draw.background().color(PLUM);

    let mut brush = Brush::new();

    // for line in reader.lines() {

    while i < img_lines.len() {
        let line = img_lines[i];
        // get image info from first line of file
        if i == 0 {
            let mut image_info = line.split_whitespace();
            res_x = image_info.next().unwrap().parse().unwrap();
            res_y = image_info.next().unwrap().parse().unwrap();
            num_colors = image_info.next().unwrap().parse().unwrap();

            color_pallet = vec![vec![0.0; 3]; num_colors];
            // set window size
            //window.set_inner_size_pixels(res_x as u32, res_y as u32);
            //window.set_outer_position_pixels(0, 0);
            
            // set origin to lower left corner
            // draw = draw.x_y(-(res_x as f32)*0.5, -(res_y as f32)*0.5);
            // println!("{} {} {}", res_x.unwrap(), res_y, num_colors);
            i=i+1;
            continue;
        }

        // store color palette
        else if i <= num_colors {
            let mut rgb_str = line.split_whitespace();
            let rgb_val = vec![
                rgb_str.next().unwrap().parse::<f32>().unwrap(),
                rgb_str.next().unwrap().parse::<f32>().unwrap(),
                rgb_str.next().unwrap().parse::<f32>().unwrap()
            ];
                
            color_pallet[i-1] = rgb_val;
            i=i+1;
            continue;
        }

        // draw with commands
        let mut command = line.split(" ");
        match command.next().unwrap() {
            "am" => {
                // put down brush
                //let mut iter = command.filter_map(|x| x.parse::<f32>().ok());
                let x = command.next().unwrap().parse::<f32>().unwrap();
                let y = command.next().unwrap().parse::<f32>().unwrap();
                brush.px_py(x, y);
                brush.x_y(x, y);
                
                //draw.rect().color(STEELBLUE).w(1.0).h(1.0).x_y(x,y);
            },
            "ad" => {
                // move brush to
                let x = command.next().unwrap().parse::<f32>().unwrap();
                let y = command.next().unwrap().parse::<f32>().unwrap();
                brush.x_y(x, y);
                draw_line(&draw, &mut brush);
            },
            "e" => {
                brush.dx_dy(1.0, 0.0);
                draw_line(&draw, &mut brush);
            },
            "f" => {
                brush.dx_dy(1.0,1.0);
                draw_line(&draw, &mut brush);
            },
            "g" => {
                brush.dx_dy(0.0,1.0);
                draw_line(&draw, &mut brush);
            },
            "h" => {
                brush.dx_dy(-1.0,1.0);
                draw_line(&draw, &mut brush);
            },
            "i" => {
                brush.dx_dy(-1.0,0.0);
                draw_line(&draw, &mut brush);
            },
            "j" => {
                brush.dx_dy(-1.0,-1.0);
                draw_line(&draw, &mut brush);
            },
            "k" => {
                brush.dx_dy(0.0,-1.0);
                draw_line(&draw, &mut brush);
            },
            "l" => {
                brush.dx_dy(1.0,-1.0);
                draw_line(&draw, &mut brush);
            },
            "nc" => {
                let color_number: usize = command.next().unwrap().parse::<usize>().unwrap();
                let color = color_pallet.get(color_number).unwrap();
                brush.color = color.to_owned();
            },
            "nb" => {
                brush.size = command.next().unwrap().parse::<usize>().unwrap();
            },
            "color" => {
                if colors_filled == true {
                    i = reader.len();
                }
                // last_stroke_index = i - 1;
                // break;
            }
            "end" => {
                colors_filled = true;
                i = num_colors;
                brush.size = 1;
                brush.color = vec![0.0; 3];
                println!("end");
            },
            _ => {},
        }
        //
        i=i+1;
        // draw.to_frame(app, &frame).unwrap();
    }

    Ok(())
}

struct Noncopyable<T>(T);

fn draw_line(draw: &Draw, brush: &mut Brush) {
    let x = Noncopyable(brush.v_x_y);
    draw.line()
        .rgb(brush.color[0], brush.color[1], brush.color[2])
        .start(brush.v_px_py)
        // .end(brush.v_x_y)
        .points(brush.v_px_py, brush.v_x_y)
        // .start(pt2(brush.px, brush.py))
        // .end(pt2(brush.x, brush.y))
        .weight(brush.size as f32);
        // .caps_round();

    brush.px_py(brush.x, brush.y);
}

