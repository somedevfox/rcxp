use std::{sync::{mpsc::*, Mutex}};
use crate::bitmap::RustBitmap;

pub static mut SFML_TX: Option<Mutex<Sender<MessageTypes>>> = None;
pub static mut RGSS_TX: Option<Mutex<Sender<MessageTypes>>> = None;

pub enum RGSSError {
    DataFolderMissing,
    ScriptsFileMissing,
    ScriptError,
    ThreadFInished
}

pub enum MessageTypes {
    RGSSThreadTerminate(RGSSError),
    BitmapCreate(u32, u32, u64),
    BitmapDispose(u64),
    SpriteCreate(u64),
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