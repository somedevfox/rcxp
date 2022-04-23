mod rcxp_window;
mod thread_common;
mod rgss_thread;
mod binding_util;

use crate::rcxp_window::*;
use crate::rgss_thread::*;
use crate::thread_common::*;
use std::sync::{mpsc::*, Mutex};

fn main() {
    println!("Creating SFML <-> RGSS channels...");
    // Create a two way channel for RGSS and SFML
    // We can still talk to either channel from sfml_tx or rgss_tx though, as we send each
    // of them a copy instead of the real deal
    // They do own their respective recievers however, so don't try to pick up messages meant
    // for them.
    let (sfml_tx, sfml_rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();
    let (rgss_tx, rgss_rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();

    println!("Spawing RGSS thread...");
    // Realistically we don't need a super complicated setup for the RGSS thread. 
    // I originally had it wrapped in a struct, but that was pretty pointless.
    let mut rgss_thread = spawn_rgss_thread(sfml_tx.clone(), rgss_rx);
    println!("Spawning SFML Window...");
    // SFML occupies the main thread. It's best to do it this way because SFML is NOT
    // thread safe.
    let mut rcxp_window = RCXPWindow::new(640, 480, "RCXP", sfml_rx, rgss_tx.clone());

    while rcxp_window.window.is_open() {
        rcxp_window.update();
    }
}
