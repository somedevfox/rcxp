use rutie::{
    Fixnum,
    RString
};
use sfml::graphics::{
    Texture
};

pub struct Bitmap {
    texture: Texture,
    filename: RString,
    width: Fixnum,
    height: Fixnum
}

impl Bitmap {
    pub fn new(w: Fixnum, h: Fixnum) -> Self {
        return Bitmap{
            texture: Texture::new(w, h),
            filename: "",
            width: w,
            height: h
        };
    }
}