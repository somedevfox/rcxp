use sfml::{*, graphics::*, window::*, system::*};
use std::{sync::{mpsc::*}};
use crate::thread_common::*;
use crate::bitmap::RustBitmap;
use std::collections::hash_map::*;

// Why does this need a lifetime specifier? I have no idea!
pub struct RCXPWindow<'a> {
    pub window: RenderWindow,
    rgss_tx: Sender<MessageTypes>,
    sfml_rx: Receiver<MessageTypes>,
    bitmap_ids: HashMap<u64, SfBox<Texture>>,
    sprite_ids: HashMap<u64, Sprite<'a>>
}

// Shorthand function to create a window. Nothing special.
pub fn create_window(width: u32, height: u32, title: &str) -> RenderWindow {
    return RenderWindow::new((width, height), title, Style::DEFAULT, &ContextSettings::default());
}

impl RCXPWindow<'_> {
    // Remember that the transmitter IS a copy. The struct owns that copy but also owns the reciever.
    // You cannot use the reciever outside of the struct.
    pub fn new(width: u32, height: u32, title: &str, sfml_rx: Receiver<MessageTypes>) -> Self {
        let window = create_window(width, height, title);
        let bitmap_ids: HashMap<u64, SfBox<Texture>> = HashMap::new();
        let sprite_ids: HashMap<u64, Sprite> = HashMap::new();
        let rgss_tx = clone_rgss_tx();

        RCXPWindow {
            window,
            rgss_tx,
            sfml_rx,
            bitmap_ids,
            sprite_ids
        }
    }

    // Update function. Checks if there are any messages in the queue, and goes through them.
    // It's not a perfect concurrency model, but its simple and it works.
    pub fn update(&mut self) {
        let message = self.sfml_rx.try_recv();
        match message {
            Err(why) => {
                if why != TryRecvError::Empty {
                    println!("RGSS Thread recv failed with error: {:?}", why)
                }
            }
            Ok(message) => {
                match message {
                    MessageTypes::BitmapCreate(w, h, id) => self.create_bitmap(w, h, id),
                    MessageTypes::BitmapDispose(id) => { self.dispose_bitmap(id) },
                    MessageTypes::BitmapCheckIfDisposed(id) => { },
                    MessageTypes::SpriteCreate(id) => self.create_sprite(id),
                    MessageTypes::SpriteDispose(id) => self.dispose_sprite(id),
                    MessageTypes::RGSSThreadTerminate(_) => { self.window.close() },
                    _ => { } // We don't need to do anything with enums such as BitmapCheckIfDisposed result here
                }
            }
        }

        self.handle_events();
    }

    // Event handler. There's not much *in* the way of events we care about, so this one is fairly simple.
    pub fn handle_events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.window.close()
            }
        }
    }

    pub fn create_bitmap(&mut self, width: u32, height: u32, bitmap_id: u64) {
        let result = Texture::new(width, height).unwrap();
        self.bitmap_ids.insert(bitmap_id, result);
    }

    pub fn dispose_bitmap(&mut self, bitmap_id: u64) {
        self.bitmap_ids.remove(&bitmap_id);
    }

    // Sends BitmapCheckIfDisposedResult to RGSS thread which contains boolean
    // true - bitmap exists
    // false - bitmap doesn't exist/was disposed of
    pub fn bitmap_check_state(&mut self, bitmap_id: u64) {
        let mut bitmap_exists = false;
        if self.bitmap_ids.contains_key(&bitmap_id) {
            bitmap_exists = true;
        } 
        self.rgss_tx.send(MessageTypes::BitmapCheckIfDisposedResult(bitmap_exists));
    }

    // Create and dispose sprites and store them in a hash associated with the sprite ID. 
    // The sprite ID is a ruby object ID passed in from the RGSS thread. This is just so we
    // can keep track of sprites easily. 
    // How graphics.update will send over sprite data I'm not sure. I'll think of something.
    pub fn create_sprite(&mut self, sprite_id: u64) {
        let sprite = Sprite::new();
        self.sprite_ids.insert(sprite_id, sprite);
    }

    pub fn dispose_sprite(&mut self, sprite_id: u64) {
        self.sprite_ids.remove(&sprite_id);
    }
}