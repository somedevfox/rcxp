extern crate sdl2;
extern crate fragile;

use fragile::Fragile;
use std::{
    thread,
    sync::{Mutex, Arc, mpsc},
    process::exit
};
use crate::window::{
    RCXPWindow
};

fn process_events(window: RCXPWindow) {
    let mut event_pump = window.sdl_context.event_pump().unwrap();
    'eventloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { exit(1); }
                _ => {}
            }
        }
    }
}

pub fn spawn(window: RCXPWindow) {
    println!("Starting RCXP Event Thread.");
    
    /*let event_thread = thread::spawn(move || {
        // let window = window_rx.recv(); // why dont this work
    });*/
    process_events(window);
}