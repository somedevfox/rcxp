use rutie::*;
use sfml::{
    SfBox
};
use sfml::graphics::{
    Texture
};

pub struct RustBitmap {
    pub id: u64,
    pub filename: String,
    pub width: u32,
    pub height: u32
}

impl RustBitmap {
    pub fn new(w: u32, h: u32) -> Self {
        return RustBitmap{
            id: 0,
            filename: "".to_string(),
            width: w,
            height: h
        };
    }
}