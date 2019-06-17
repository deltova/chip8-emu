extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas; 
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::time::Duration;
use std::process;

fn matchkey(keycode: Keycode, state: &mut bool) {
    match keycode {
        Keycode::Escape => *state = false,
        _ => println!("Other"),
    }
}
static SCALE: u32 = 5;

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


pub fn draw_pixel(pixel: bool, x: u8, y: u8, canvas: &mut Canvas<Window>) -> Rect {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut j = (y as u32) * SCALE;
    let mut i = (x as u32) * SCALE;
    Rect::new( i as i32, j as i32, SCALE, SCALE)
}
