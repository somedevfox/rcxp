use std::{sync::{mpsc::*}};

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