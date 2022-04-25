use crate::thread_common::*;
use crate::binding_util::*;
use crate::binding_util;
use flate2::read::ZlibDecoder;
use std::{thread, sync::mpsc::*, path::Path};
use std::io::{
    Read,
    Write
};
use rutie::*;

pub static mut RGSS_RX : Option<Receiver<MessageTypes>> = None;

pub fn spawn_rgss_thread(rgss_rx: Receiver<MessageTypes>) -> thread::JoinHandle<()> {
    let thread = thread::spawn(move || {
        let sfml_tx = clone_sfml_tx(); // Shadow transmitters
        unsafe { RGSS_RX = Some(rgss_rx) }

        // Set up the VM
        VM::init();
        VM::init_loadpath();
        VM::eval("$RCXP = true").unwrap();

        binding_util::bind_all();

        VM::eval(concat!(
            "bitmap = Bitmap.new(50, 30)\n",
            "puts \"Is Bitmap disposed? #{bitmap.disposed?}\"\n",
            "bitmap.dispose\n",
            "puts \"Is Bitmap disposed? #{bitmap.disposed?}\"\n",
            "puts \"Disposing of Bitmap again even if it has been disposed of.\"\n",
            "bitmap.dispose\n"
        )).unwrap();

        loop {}

        // Run the RGSS Scripts
        let error = run_rgss_scripts();
        // Notify the SFML thread that thread processing is finished and we're about to Ack
        let result = sfml_tx.send(MessageTypes::RGSSThreadTerminate(error));
        process_send_result(result);
        // Ack
    });
    return thread; 
}

fn run_rgss_scripts() -> RGSSError {
    if !Path::new("Data").exists() {
        println!("Data folder missing.");
        return RGSSError::DataFolderMissing;
    }

    if !Path::new("Data/Scripts.rxdata").exists() {
        println!("Scripts file missing.");
        return RGSSError::ScriptsFileMissing;
    }

    // Get the File class
    let rb_file_class = Class::from_existing("File");
    // Open the Scripts file
    let script_file = rb_file_class.protect_send("open", &[
        AnyObject::from(&RString::new_utf8("Data/Scripts.rxdata")),
        AnyObject::from(&RString::new_utf8("rb"))
    ]).unwrap();
    // Try to convert it to an array
    let arr = anyobj_to_array(marsh_load(script_file), "Error loading script data:");
    // Loop over all array items
    for item in arr.into_iter() {
        // Try to convert array item *to* an array
        let arr = anyobj_to_array(item, "Error loading script data:");
        // Get name and contents
        let name = anyobj_to_rstring(arr.at(1), "Error loading script name:");
        let contents = anyobj_to_rstring(arr.at(2), "Error loading script contents:");
        // Convert contents to a byte buffer
        let zlib_bytes = contents.to_vec_u8_unchecked();
        // Decode the byte buffer (it's encoded with zlib)
        let mut z = ZlibDecoder::new(&zlib_bytes[..]);
        let mut s = String::new();
        z.read_to_string(&mut s);

        // Eval script, and exit the thread if it errors
        let result = VM::eval(&s);
        match result {
            Err(why) => { 
                println!("RGSS Error: {}", why);
                return RGSSError::ScriptError;
            }
            _ => {}
        }
    }
    RGSSError::ThreadFinished
}