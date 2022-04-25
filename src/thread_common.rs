use std::{sync::{mpsc::*, Mutex}};
use crate::bitmap::RustBitmap;

pub static mut SFML_TX: Option<Mutex<Sender<MessageTypes>>> = None;
pub static mut SFML_RX: Option<Mutex<Receiver<MessageTypes>>> = None;
pub static mut RGSS_TX: Option<Mutex<Sender<MessageTypes>>> = None;
pub static mut RGSS_RX: Option<Mutex<Receiver<MessageTypes>>> = None;

pub enum RGSSError {
    DataFolderMissing,
    ScriptsFileMissing,
    ScriptError,
    ThreadFinished,
    BitmapCreationError
}

pub enum MessageTypes {
    /*
    * RGSSThreadTerminate
    * > When thread encounters any exception (such as uninitialized constant or Scripts.rxdata not found)
    * > It will print out error, then send a message with enum of RGSSThreadTerminate which has an argument 
    * > of RGSSError (see enum above)
    * > rcxp_window.rs will receive RGSSThreadTerminate enum and will simply close window.
    * 
    * Arguments:
    *     RGSSThread - error
    */
    RGSSThreadTerminate(RGSSError),

    /* 
    * BitmapCreate
    * > Creates SFML Texture and stores it in bitmap_ids HashMap in rcxp_window.rs.
    * > Does not return anything to sender. // TODO: send some kind of result back to sender for creation verification
    * 
    * Arguments:
    *     1st u32 - width
    *     2nd u32 - height
    *     u64     - bitmap id
    */
    BitmapCreate(u32, u32, u64),

    /*
    * BitmapResult
    * > When bitmap is created, it will send a message with enum of BitmapResult which has an argument
    * > of Result<(), RGSSError> (see enum above)
    * 
    * Arguments:
    *     Result - result
    */
    BitmapResult(Result<(), RGSSError>),

    /* 
    * BitmapDispose
    * > Deletes SFML Texture and removes it from bitmap_ids HashMap in rcxp_window.rs.
    * > If Bitmap has been already disposed of, will do nothing.
    * > Does not return anything to sender.
    * 
    * Arguments:
    *     u64 - bitmap id
    */
    BitmapDispose(u64),

    /*
    * BitmapCheckIfDisposed
    * > Checks if bitmap exists in bitmap_ids HashMap in rcxp_window.rs,
    * > Then, returns BitmapCheckIfDisposedResult enum to sender.
    *
    * BitmapCheckIfDisposed Arguments:
    *     u64  - bitmap id
    * BitmapCheckIfDisposedResult Arguments:
    *     bool - bool which tells whether bitmap exists or not
    */
    BitmapCheckIfDisposed(u64),
    BitmapCheckIfDisposedResult(bool),

    /*
    * SpriteCreate
    * > Creates SFML Sprite and stores it in sprite_ids HashMap in rcxp_window.rs.
    * > Does not return anything to sender. // TODO: send some kind of result back to sender for creation verification
    * Arguments:
    *     u64 - sprite id
    */
    SpriteCreate(u64),

    /* 
    * SpriteDispose
    * > Deletes SFML Sprite and removes it from sprite_ids HashMap in rcxp_window.rs.
    * > If Sprite has been already disposed of, will do nothing.
    * > Does not return anything to sender.
    * 
    * Arguments:
    *     u64 - sprite id
    */
    SpriteDispose(u64)
}

pub fn process_send_result(result: std::result::Result<(), SendError<MessageTypes>>) {
    match result {
        Ok(_ok) => {}
        Err(why) => {
            panic!("Sent message errored with: {:?}", why);
        }
    }
}

pub fn process_recv_result(result: std::result::Result<MessageTypes, RecvError>) -> MessageTypes {
    match result {
        Ok(message) => { message }
        Err(why) => {
            panic!("Recv result errored with {:?}", why);
        }
    }
}

pub fn clone_rgss_tx() -> Sender<MessageTypes> {
    unsafe { RGSS_TX.as_ref().unwrap().lock().unwrap().clone() }
}

pub fn clone_sfml_tx() -> Sender<MessageTypes> {
    unsafe { SFML_TX.as_ref().unwrap().lock().unwrap().clone() }
}