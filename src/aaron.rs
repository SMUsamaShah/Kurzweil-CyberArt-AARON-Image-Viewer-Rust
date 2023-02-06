use std::{fs::File};
// use std::io::{self, BufReader};
use std::io::Read;

// mod aaron {
// }

pub struct AaronImage {
    res_x: u16,
    res_y: u16,
    colors: Vec<Vec<f32>>
}
impl AaronImage {
    pub fn new() -> Self {
        Self{
            res_x: 0,
            res_y: 0,
            colors: vec![vec![0.0; 3]; 0],
            // cb_img_info: Box::new(|x,y,col|{})
        }
    }
}

pub struct DrawCallbacks<'callback> {
    pub img_info: Box<dyn FnMut(u16, u16, &Vec<Vec<f32>>) + 'callback>,
    pub draw_line_ab: Box<dyn FnMut(f32, f32, f32, f32) + 'callback>,
    pub line_color: Box<dyn FnMut(f32, f32, f32) + 'callback>,
    pub line_width: Box<dyn FnMut(usize) + 'callback>,
}
impl DrawCallbacks<'_> {
    pub fn img_info(&mut self, img: &AaronImage) {
        (self.img_info)(img.res_x, img.res_y, &img.colors)
    }
    pub fn draw_line_ab(&mut self, brush: &mut Brush) {
        (self.draw_line_ab)(brush.px, brush.py, brush.x, brush.y);
        brush.px = brush.x;
        brush.py = brush.y;
    }
    pub fn line_color(&mut self, brush: &Brush) {
        (self.line_color)(brush.color[0], brush.color[1], brush.color[2])
    }
    pub fn line_width(&mut self, brush: &Brush) {
        (self.line_width)(brush.size)
    }
    pub fn new()-> Self {
        Self {
            img_info: Box::new(|x,y,col|{}),
            draw_line_ab: Box::new(|x1,y1,x2,y2|{}),
            line_color: Box::new(|a,b,c|{}),
            line_width: Box::new(|a|{}),
        }
    }
}

struct Brush {
    px: f32,
    py: f32,
    x: f32,
    y: f32,
    size: usize,
    color: Vec<f32>,
}
impl Brush {
    pub fn new() -> Self {
        Self {
            px: 0.0, py: 0.0, 
            x: 0.0, y: 0.0, 
            size: 1, 
            color: vec![0.0 as f32; 3],
        }
    }
    pub fn px_py(&mut self, px : f32, py : f32) {
        self.px = px;
        self.py = py;
    }
    pub fn x_y(&mut self, x : f32, y : f32) {
        self.x = x;
        self.y = y;
    }
    pub fn dx_dy(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}

pub fn read_file(aaron_img_file_path: &str, draw_callbacks: &mut DrawCallbacks) {
    println!("read image");
    let file = File::open(aaron_img_file_path);
    let mut file_text = String::new();
    let _ = file.unwrap().read_to_string(&mut file_text);
// }

// fn parse(img_data: &str, draw_callbacks: &DrawCallbacks) {
    let mut img = AaronImage::new();
    let img_lines = file_text.lines().collect::<Vec<_>>();
    let mut num_colors = 0;
    let mut brush = Brush::new();
    let mut redraw_lines = false;

    let n = img_lines.len();
    let mut i = 0;
    while i < n {
        let line = img_lines[i];
        if i == 0 {
            let mut image_info = line.split_whitespace();
            img.res_x = image_info.next().unwrap().parse().unwrap();
            img.res_y = image_info.next().unwrap().parse().unwrap();
            num_colors = image_info.next().unwrap().parse().unwrap();

            img.colors = vec![vec![0.0; 3]; num_colors];

            i += 1;
            continue;
        }
        else if i <= num_colors {
            let mut rgb_str = line.split_whitespace();
            let rgb_val = vec![
                rgb_str.next().unwrap().parse::<f32>().unwrap(),
                rgb_str.next().unwrap().parse::<f32>().unwrap(),
                rgb_str.next().unwrap().parse::<f32>().unwrap()
            ];
            img.colors[i-1] = rgb_val;

            i += 1;
            continue;
        }
        else if i == num_colors + 1 {
            draw_callbacks.img_info(&img);
        }

        let mut command = line.split(" ");
        let cmd = command.next().unwrap();
        match cmd {
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
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "e" => {
                brush.dx_dy(1.0, 0.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "f" => {
                brush.dx_dy(1.0,1.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "g" => {
                brush.dx_dy(0.0,1.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "h" => {
                brush.dx_dy(-1.0,1.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "i" => {
                brush.dx_dy(-1.0,0.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "j" => {
                brush.dx_dy(-1.0,-1.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "k" => {
                brush.dx_dy(0.0,-1.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "l" => {
                brush.dx_dy(1.0,-1.0);
                draw_callbacks.draw_line_ab(&mut brush);
            },
            "nc" => {
                let color_number: usize = command.next().unwrap().parse::<usize>().unwrap();
                let color = img.colors.get(color_number).unwrap();
                brush.color = color.to_owned();
                draw_callbacks.line_color(&brush);
            },
            "nb" => {
                brush.size = command.next().unwrap().parse::<usize>().unwrap();
                draw_callbacks.line_width(&brush);
            },
            "color" => {
                println!("{} {}",cmd,i);
                if redraw_lines == true {
                    i = n; // jump to end
                }
                // last_stroke_index = i - 1;
                // break;
                println!("{} {}",cmd,i);
            }
            "end" => {
                println!("{} {}",cmd,i);
                i = num_colors; // jump to line drawing

                redraw_lines = true;
                brush.size = 1;
                brush.color = vec![0.0; 3];
                println!("{} {}",cmd,i);
            },
            _ => {
                println!("the end");
            }
        }
        i += 1;
    }
}