extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::render::Canvas; 
use sdl2::video::Window;
use sdl2::rect::Rect;

pub fn init_sdl(scale : u8) -> (Canvas<Window>, sdl2::Sdl)
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("chip8",  (scale as u32) * 64, (scale as u32)* 32) 
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    (canvas, sdl_context)
}


pub fn draw_pixel(x: u8, y: u8, canvas: &mut Canvas<Window>, scale: u8) -> Rect {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let j = (y as u32) * (scale as u32);
    let i = (x as u32) * (scale as u32);
    Rect::new( i as i32, j as i32, scale as u32, scale as u32)
}
