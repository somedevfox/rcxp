use std::{sync::{mpsc::*}};

pub enum RGSSError {
    DataFolderMissing,
    ScriptsFileMissing,
    ThreadAck
}

pub enum MessageTypes {
    RGSSThreadTerminate(RGSSError),
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