use crate::thread_common::*;

use std::{thread, sync::mpsc::*};
use rutie::*;

pub struct RGSSThread {
    pub thread: thread::JoinHandle<()>,
    tx: Sender<MessageTypes>,
    rx: Receiver<MessageTypes>,
    sfml_tx: Sender<MessageTypes>,
    rgss_rx: Receiver<MessageTypes>
}

impl RGSSThread {
    pub fn new(sfml_tx: Sender<MessageTypes>, rgss_rx: Receiver<MessageTypes>) -> Self {
        let (tx, thread_rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();
        let (thread_tx, rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();

        let thread = thread::spawn(move || {
            let (thread_tx, thread_rx) = (thread_tx, thread_rx);

            VM::init();

            VM::eval("$RCXP = true");

            loop {}
        });

        RGSSThread {
            thread,
            tx,
            rx,
            sfml_tx,
            rgss_rx
        }
    }

    pub fn update(&mut self) {
        let message = self.rx.try_recv();
        match message {
            Err(why) => {
                if why != TryRecvError::Empty {
                    println!("RGSS Thread recv failed with error: {:?}", why)
                }
            }
            Ok(_) => {}
        }
    }
}