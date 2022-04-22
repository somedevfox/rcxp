mod rcxp_window;
mod thread_common;
mod rgss_thread;

use crate::rcxp_window::*;
use crate::rgss_thread::*;
use crate::thread_common::*;
use std::sync::mpsc::*;

fn main() {
    let (sfml_tx, sfml_rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();
    let (rgss_tx, rgss_rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();


    let mut rgss_thread = RGSSThread::new(sfml_tx, rgss_rx);
    let mut rcxp_window = RCXPWindow::new(640, 480, "RCXP", sfml_rx, rgss_tx);

    while rcxp_window.window.is_open() {
        rcxp_window.update();
        rgss_thread.update();
    }
}
