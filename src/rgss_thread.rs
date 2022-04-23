use crate::thread_common::*;

use std::{thread, sync::mpsc::*};
use rutie::*;

pub fn spawn_rgss_thread(sfml_tx: Sender<MessageTypes>, rgss_rx: Receiver<MessageTypes>) -> thread::JoinHandle<()> {
        let thread = thread::spawn(move || {
            let (sfml_tx, rgss_rx) = (sfml_tx, rgss_rx); // Shadow transmitters

            VM::init();

            VM::eval("$RCXP = true");

            loop {}
        });
        return thread;
    }
