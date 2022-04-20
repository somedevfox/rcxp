extern crate sdl2;

use std::{
    process,
    thread,
    sync::mpsc::channel
};
use crate::window::{
    RCXPWindow
};

fn process_events(window: RCXPWindow) {

}

pub fn spawn(window: RCXPWindow) {
    println!("Starting RCXP Event Thread.");
    let (tx, rx) = channel();
    tx.send(window);
    
    let th1 = thread::spawn(move || {
        process_events();
    });
    th1.join().expect("Event Thread has panicked and RCXP will now quit.");
}