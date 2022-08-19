use std::alloc::dealloc;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, mpsc, Mutex};
use sdl2::pixels::{Color, PixelFormat};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{mem, thread};
use std::thread::JoinHandle;
use sdl2::libc::{abs, c_int, exit, tm};
use sdl2::rect::Point;
use sdl2::surface::Surface;

const width:usize = 500;
const uwidth:u32 = width as u32;
const height:usize = 500;
const uheight:u32 = height as u32;

const size:usize = width*height;


fn main() {
    let sdl_context = sdl2::init().expect("Sdl init failed.");
    let video_subsystem = sdl_context.video().expect("VideoSubsytem failed");

    let window = video_subsystem.window("rust-sdl2 hello", uwidth, uheight)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("coudn't make canvas T-T");

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().expect("Could not aquire events?");


    //prerequisites
    const maxit: i32 = 2;
    let mut draw:bool=true;
    // let mut pixels : Vec<Arc<Mutex<HashMap<usize,Color>>>> = vec![Arc::new(Mutex::new(HashMap::from(tmp)))];
    // let mut threads : vec<[]> = ();
    let (sender, receiver) = mpsc::channel();
    'main: loop {
        //calculate
        //draw
        println!("HIII");
        if draw {
            // let pixels:[Color; (width * height) as usize] = [];
            for i in 0..size {
                println!("making thread for pixel {i}");
                let senderr = sender.clone();
                // let pixels = Arc::clone(&pixels);
                std::thread::spawn(move || unsafe {
                    let x = i%width;
                    let y = i/height;
                    let mut brightness = 0;

                    for x in 0..width {
                        for y in 0..height {
                            let mut a = (width - 50) / 50;
                            let mut b = (height - 50) / 50;

                            let ca = a;
                            let cb = b;

                            let mut n = 0;
                            let z = 0;

                            while abs(n) < maxit {
                                let real = a * a - b * b;
                                let complex = 2 * a * b;

                                a = real + ca;
                                b = complex + cb;

                                if abs((a + b) as c_int) > maxit {
                                    break;
                                }

                                n += 1;
                            }
                            if n == 0 {
                                brightness = 0;
                            } else {
                                brightness = n / 100 * 255 % 255;
                            }
                            if n == 80 { brightness = 255; } else if n == maxit { brightness = 0; }
                        }
                    }
                    senderr.send((Point::new((i % width) as i32, (i / height) as i32), Color::RGB(brightness as u8, brightness as u8, brightness as u8))).unwrap();
                }).join().expect("Couldn't join thread {i}");

            }
            // draw = false;

            println!("Hello");
            mem::drop(&sender);
            let mut i = 0;
            for received in &receiver {
                // println!("Got also {:?}",received);
                let (p, c) = received;

                canvas.set_draw_color(c);
                canvas.draw_point(p).unwrap();
                canvas.clear();
                canvas.present();
                println!("boom");
                i+=1;
                if i==size {break;}
            }
        }
        //get keys
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },
                _ => {}
            }
        }
    }


    println!("Hello, world!");
}