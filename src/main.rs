use std::io::{stdout, Write};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

const WIDTH:usize = 500;
const UWIDTH:u32 = WIDTH as u32;
const HEIGHT:usize = 500;
const UHEIGHT:u32 = HEIGHT as u32;


fn main() {
    //basic setup
    let sdl_context = sdl2::init().expect("Sdl init failed.");
    let mut events = sdl_context.event_pump().expect("Could not acquire events?");
    let video_subsystem = sdl_context.video().expect("VideoSubsystem failed");
    let window = video_subsystem.window("rust-sdl2 hello", UWIDTH, UHEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build().expect("couldn't make canvas T-T");
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();
    let mut screen = ScreenWindow::new(-2.0, 2.0, -2.0, 2.0);


    //variables
    let mut max_iterations:f64 = 120.0;
    drawf(&mut canvas, &screen, max_iterations);
    let mut line = String::new();
    let stdin = std::io::stdin();


    const SPEED:f64 = 0.03;
    const ZOOM:f64 = 0.08;
    'main: loop {
        //rect to events in event queue
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },
                Event::KeyDown {keycode: Some(Keycode::Up), .. } => {
                    screen.pan(0.0,(screen.right-screen.left)*SPEED);
                    drawf(&mut canvas, &screen, max_iterations);
                },
                Event::KeyDown {keycode: Some(Keycode::Down), .. } => {
                    screen.pan(0.0,(screen.right-screen.left)*-SPEED);
                    drawf(&mut canvas, &screen, max_iterations);
                },
                Event::KeyDown {keycode: Some(Keycode::Left), .. } => {
                    screen.pan((screen.right-screen.left)*-SPEED,0.0);
                    drawf(&mut canvas, &screen, max_iterations);
                },
                Event::KeyDown {keycode: Some(Keycode::Right), .. } => {
                    screen.pan((screen.right-screen.left)*SPEED,0.0);
                    drawf(&mut canvas, &screen, max_iterations);
                },
                Event::MouseWheel {y:0..=i32::MAX, .. } => {
                    screen.zoom((screen.right-screen.left)*ZOOM);
                    drawf(&mut canvas, &screen, max_iterations);
                }
                Event::MouseWheel {y:i32::MIN..=0, .. } => {
                    screen.zoom((screen.right-screen.left)*-ZOOM);
                    drawf(&mut canvas, &screen, max_iterations);
                },
                Event::KeyDown {keycode: Some(Keycode::Equals), .. } => {
                    max_iterations +=20.0;
                    drawf(&mut canvas, &screen, max_iterations);
                }
                Event::KeyDown {keycode: Some(Keycode::Minus), .. } => {
                    max_iterations -=20.0;
                    drawf(&mut canvas, &screen, max_iterations);
                }
                Event::KeyDown {keycode: Some(Keycode::Z), .. } => {
                    print!("Enter Zoom Level:");
                    stdout().flush().unwrap();
                    stdin.read_line(&mut line).unwrap();
                    line.truncate(line.len()-1);
                    println!("{:?}",line);
                    screen.zoom= line.parse::<f64>().unwrap();
                    line.clear();
                    screen.zoom((screen.right-screen.left)*ZOOM*screen.zoom);
                    drawf(&mut canvas, &screen, max_iterations);
                }
                Event::KeyDown {keycode: Some(Keycode::P), .. } => {
                    print!("Enter Pan X:");
                    stdout().flush().unwrap();
                    stdin.read_line(&mut line).unwrap();
                    line.truncate(line.len()-1);
                    screen.pan.0= line.parse::<f64>().unwrap();
                    line.clear();
                    print!("Enter Pan Y:");
                    stdout().flush().unwrap();
                    stdin.read_line(&mut line).unwrap();
                    line.truncate(line.len()-1);
                    screen.pan.1= line.parse::<f64>().unwrap();
                    line.clear();
                    screen.pan((screen.right-screen.left)*SPEED*screen.pan.0,(screen.right-screen.left)*SPEED*screen.pan.1);
                    drawf(&mut canvas, &screen, max_iterations);
                }
                Event::AppLowMemory { .. } => {
                    break 'main;
                }
                _ => {}
            }
        }
    }
    println!("Goodbye");
}

//p5.js's map range function
fn _map(n:f64,start1:f64,stop1:f64,start2:f64,stop2:f64) -> f64 {
    return (n - start1) / (stop1 - start1) * (stop2 - start2) + start2;
}

const PRINT_POSITION: bool = true;
const PRINT_DRAWING_STATUS: bool = false;
const BRIGHTNESS_SCALE: f64 = 20.0;//0-20 to 0-255
//drawfractal()
fn drawf(canvas:&mut WindowCanvas, s: &ScreenWindow, maxit:f64) {
    if PRINT_DRAWING_STATUS {
        println!("Drawing fractal");
    }
    if PRINT_POSITION {
        println!("{:?}",s);
    }
    canvas.clear();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            //a and b are x,y coordinates that need to be mapped to where the screen is zoomed into
            let mut a = _map(x as f64, 0.0, WIDTH as f64, s.left, s.right);
            let mut b = _map(y as f64, 0.0, HEIGHT as f64, s.bottom, s.top);

            //copies of a and b
            let ca = a;
            let cb = b;

            //number of iterations(it's a float so we don't have to convert
            let mut n:f64 = 0.0;

            while n < maxit {
                let real = (a*a) - (b*b);
                let complex = 2.0*a*b;

                a = real + ca;
                b = complex + cb;

                if a.abs() + b.abs() > 100.0 {
                    break;
                }

                n += 1.0;
            }

            let mut brightness = _map(n, 0.0, BRIGHTNESS_SCALE, 0.0, 255.0);
            if n == maxit { brightness = 0.0; }
            canvas.set_draw_color(Color::RGB(brightness as u8, brightness as u8, brightness as u8));
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }
    canvas.present();
    if PRINT_DRAWING_STATUS {
        println!("done");
    }
    return;
}

#[derive(Debug)]
struct ScreenWindow {
    left:f64,
    right:f64,
    top:f64,
    bottom:f64,
    zoom:f64,     //zoom and pan are there so you can easily get back
    pan:(f64,f64) //to where you were (x,y)
}
impl ScreenWindow {
    pub fn new(l:f64,r:f64,t:f64,b:f64) -> Self {
        Self{left:l,right:r,bottom:b,top:t,zoom:0.0,pan:(0.0,0.0)}
    }
    pub fn zoom(&mut self, amount:f64) {
        self.left+=amount;
        self.right-=amount;
        self.top+=amount;
        self.bottom-=amount;
        self.zoom+=amount;
    }
    pub fn pan(&mut self,x:f64,y:f64) {
        self.left+=x;
        self.right+=x;
        self.top+=y;
        self.bottom+=y;
        self.pan.0+=x;
        self.pan.1+=y;
    }
}

