use crate::thread_common::*;

use std::{thread, sync::{mpsc::*}};
use sfml::{graphics::*, window::*, system::*};

pub struct SFMLThread {
    pub thread: thread::JoinHandle<()>,
    pub tx: Sender<MessageTypes>,
    pub rx: Receiver<MessageTypes>,
}

impl SFMLThread {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        let (tx, thread_rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();
        let (thread_tx, rx): (Sender<MessageTypes>, Receiver<MessageTypes>) = channel();

        let thread = thread::spawn(move || {
            let (thread_tx, thread_rx) = (thread_tx, thread_rx);

            let mut window = RenderWindow::new((width, height), &title, Style::DEFAULT, &ContextSettings::default());

            loop {
                let message = thread_rx.try_recv();
                match message {
                    Err(why) => {
                        if why != TryRecvError::Empty {
                            println!("SFML Thread recv failed with error: {:?}", why)
                        }
                    }
                    Ok(message) => {
                        match message {
                            MessageTypes::WindowOpen(_) => { 
                                let result = thread_tx.send(MessageTypes::WindowOpen(window.is_open())); 
                                process_send_result(result);
                            }
                        }
                    }
                }

                while let Some(event) = window.poll_event() {
                    if event == Event::Closed {
                        window.close();
                    }
                }
            }
        });

        SFMLThread {
            thread,
            tx,
            rx
        }
    }

    pub fn update(&mut self) {
        let message = self.rx.try_recv();
        match message {
            Err(why) => {
                if why != TryRecvError::Empty {
                    println!("SFML Thread recv failed with error: {:?}", why)
                }
            }
            Ok(message) => {
            }
        }
    }

    pub fn window_open(&self) -> bool {
        let result = self.tx.send(MessageTypes::WindowOpen(false));
        process_send_result(result);
        let message = process_recv_result(self.rx.recv());
        match message {
            MessageTypes::WindowOpen(open) => { open }
        }
    }
}