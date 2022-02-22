extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

//let sdl_context;

pub fn CreateWindow(title:&str, width:u32, height:u32) {
    let sdl_context = sdl2::init().unwrap();
    let vs = sdl_context.video().unwrap();

    let window = vs.window(title, width, height)
        .position_centered()
        .build()
        .unwrap();

}