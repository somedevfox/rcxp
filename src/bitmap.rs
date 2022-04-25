use rutie::*;
use sfml::{ SfBox, graphics::{ Texture } };

pub struct RustBitmap {
    pub filename: String,
    pub width: u32,
    pub height: u32
}

impl RustBitmap {
    pub fn new(w: u32, h: u32) -> Self {
        return RustBitmap{
            filename: "".to_string(),
            width: w,
            height: h
        };
    }
}