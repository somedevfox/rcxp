use rutie::{
    Fixnum,
    RString,
    VM,
    Class,
    Object,
    AnyObject
};
use crate::bitmap::RustBitmap;
use crate::thread_common;

wrappable_struct!(RustBitmap, RustBitmapWrapper, RUSTBITMAP_WRAPPER);
class!(Bitmap);
methods!(
    Bitmap, 
    itself,

    fn bitmap_new(w: Fixnum, h: Fixnum) -> AnyObject {
        let mut bitmap = RustBitmap::new(w.unwrap().to_i64() as u32, h.unwrap().to_i64() as u32);
        
        let class = Class::from_existing("Bitmap");
        let result = class.protect_send("object_id", &[]);
        match result {
            Err(why) => { VM::raise(Class::from_existing("StandardError"), "Failed to get Object ID"); },
            Ok(data) => {
                bitmap.id = data.try_convert_to::<Fixnum>().unwrap().to_i64() as u64;
            }
        }

        class.wrap_data(bitmap, &*RUSTBITMAP_WRAPPER)
    }
);

pub fn bind(tx: std::sync::mpsc::Sender<thread_common::MessageTypes>) {
    Class::new("Bitmap", Some(&Class::from_existing("Object"))).define(|itself| {
        itself.def_self("new", bitmap_new);
    });
}