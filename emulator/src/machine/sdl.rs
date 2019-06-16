extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas; 
use sdl2::video::Window;
use sdl2::rect::Point;
use std::time::Duration;
use std::process;

fn matchkey(keycode: Keycode, state: &mut bool) {
    match keycode {
        Keycode::Escape => *state = false,
        _ => println!("Other"),
    }
}
static SCALE: u32 = 3;

pub fn init_sdl() -> (Canvas<Window>, sdl2::Sdl)
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("chip8",  (SCALE as u32) * 64, (SCALE as u32)* 32) 
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    (canvas, sdl_context)
}


pub fn draw_pixel(pixel: bool, x: u8, y: u8, canvas: &mut Canvas<Window>) {
    if pixel {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    }
    else {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }
    let mut j = (y as u32) * SCALE;
    for _ in 0..SCALE {
        let mut i = (x as u32) * SCALE;
        for _ in 0..SCALE {
            let point = Point::new( i as i32, j as i32);
            canvas.draw_point(point);
            canvas.present();
            i += 1
        }
        j += 1
    }
}
