extern crate sdl2;

mod window;
mod eventthread;

use std::{
    thread,
    sync::mpsc
};

use crate::window::{
    RCXPWindow
};

fn main() {
    let win = window::create_window("rcxp", 640, 480);

    eventthread::spawn(win);
}
