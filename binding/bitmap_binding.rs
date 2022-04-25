use rutie::{
    Fixnum,
    RString,
    VM,
    Class,
    Object,
    Boolean,
    NilClass,
    AnyObject
};
use crate::bitmap::RustBitmap;
use crate::thread_common::*;
use crate::clone_sfml_tx;
use crate::rgss_thread::RGSS_RX;

wrappable_struct!(RustBitmap, RustBitmapWrapper, RUSTBITMAP_WRAPPER);
class!(Bitmap);
methods!(
    Bitmap, 
    itself,

    fn bitmap_new(w: Fixnum, h: Fixnum) -> AnyObject {
        let width = w.unwrap().to_i64() as u32;
        let height = h.unwrap().to_i64() as u32;
        let mut bitmap = RustBitmap::new(width, height);
        
        let class = Class::from_existing("Bitmap");
        let obj: AnyObject = class.wrap_data(bitmap, &*RUSTBITMAP_WRAPPER);
        let mut id: u64 = 0;

        let result = obj.protect_send("object_id", &[]);
        match result {
            Err(_why) => { VM::raise(Class::from_existing("StandardError"), "Failed to get Object ID"); },
            Ok(data) => {
                id = data.try_convert_to::<Fixnum>().unwrap().to_i64() as u64;
            }
        }
        let sfml_tx = clone_sfml_tx();
        sfml_tx.send(MessageTypes::BitmapCreate(width, height, id));

        obj
    }

    fn bitmap_dispose() -> AnyObject {
        let id = itself.protect_send("object_id", &[])
                        .unwrap()
                        .try_convert_to::<Fixnum>()
                        .unwrap().to_i64() as u64;
        let sfml_tx = clone_sfml_tx();
        sfml_tx.send(MessageTypes::BitmapDispose(id));
        NilClass::new().to_any_object()
    } 

    fn bitmap_disposed() -> AnyObject {
        let mut disposed = false;
        let id = itself.protect_send("object_id", &[])
                        .unwrap()
                        .try_convert_to::<Fixnum>()
                        .unwrap().to_i64() as u64;
        let sfml_tx = clone_sfml_tx();
        let result = sfml_tx.send(MessageTypes::BitmapCheckIfDisposed(id));
        process_send_result(result);

        let msg: MessageTypes;
        unsafe {
            match RGSS_RX.as_ref().unwrap().recv() {
                Err(why) => {
                    panic!("Bitmap: Failed to get message from SFML Thread");
                },
                Ok(m) => msg = m
            };
        }
        
        match msg {
            MessageTypes::BitmapCheckIfDisposedResult(d) => disposed = d,
            _ => { panic!("Bitmap: Got wrong message"); }
        }

        Boolean::new(disposed).to_any_object()
    }
);

pub fn bind() {
    Class::new("Bitmap", Some(&Class::from_existing("Object"))).define(|itself| {
        itself.def_self("new", bitmap_new);

        itself.def("dispose", bitmap_dispose);
        itself.def("disposed?", bitmap_disposed);
    });
}