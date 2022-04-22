use sfml::{graphics::*, window::*, system::*};
use std::{sync::{mpsc::*}};
use crate::thread_common::*;
use std::collections::hash_map::*;

pub struct RCXPWindow<'a> {
    pub window: RenderWindow,
    rgss_tx: Sender<MessageTypes>,
    sfml_rx: Receiver<MessageTypes>,
    sprite_ids: HashMap<u64, Sprite<'a>>
}

pub fn create_window(width: u32, height: u32, title: &str) -> RenderWindow {
    return RenderWindow::new((width, height), title, Style::DEFAULT, &ContextSettings::default());
}

impl RCXPWindow<'_> {
    pub fn new(width: u32, height: u32, title: &str, sfml_rx: Receiver<MessageTypes>, rgss_tx: Sender<MessageTypes>) -> Self {
        let window = create_window(width, height, title);
        let sprite_ids: HashMap<u64, Sprite> = HashMap::new();

        RCXPWindow {
            window,
            rgss_tx,
            sfml_rx,
            sprite_ids
        }
    }

    pub fn update(&mut self) {
        self.handle_events();
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.window.close()
            }
        }
    }

    pub fn create_sprite(&mut self, sprite_id: u64) {
        let sprite = Sprite::new();
        self.sprite_ids.insert(sprite_id, sprite);
    }

    pub fn dispose_sprite(&mut self, sprite_id: u64) {
        self.sprite_ids.remove(&sprite_id);
    }
}