use crate::thread_common::*;
use crate::binding_util::*;
use flate2::read::ZlibDecoder;
use std::{thread, sync::mpsc::*, fs, path::*};
use std::io::{
    Read,
    Write
};
use rutie::*;

pub fn spawn_rgss_thread(sfml_tx: Sender<MessageTypes>, rgss_rx: Receiver<MessageTypes>) -> thread::JoinHandle<()> {
    let thread = thread::spawn(move || {
        let (sfml_tx, rgss_rx) = (sfml_tx, rgss_rx); // Shadow transmitters

        VM::init();
        VM::init_loadpath();
        VM::eval("$RCXP = true");

        run_rgss_scripts();
        sfml_tx.send(MessageTypes::RGSSThreadTerminate(RGSSError::ThreadAck));
    });
    return thread; 
}

fn run_rgss_scripts() {
    let rb_file_class = Class::from_existing("File");
    let script_file = rb_file_class.protect_send("open", &[
        AnyObject::from(&RString::new_utf8("Data/Scripts.rxdata")),
        AnyObject::from(&RString::new_utf8("rb"))
    ]).unwrap();

    let arr = anyobj_to_array(marsh_load(script_file), "Error loading script data:");
    for item in arr.into_iter() {
        let arr = anyobj_to_array(item, "Error loading script data:");

        let name = anyobj_to_rstring(arr.at(1), "Error loading script name:");
        let contents = anyobj_to_rstring(arr.at(2), "Error loading script contents:");
        let zlib_bytes = contents.to_vec_u8_unchecked();

        let mut z = ZlibDecoder::new(&zlib_bytes[..]);
        let mut s = String::new();
        z.read_to_string(&mut s);

        let result = VM::eval(&s);
        match result {
            Err(why) => { 
                println!("RGSS Error: {}", why);
                return;
            }
            _ => {}
        }
    }
}