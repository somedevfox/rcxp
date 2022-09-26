use crate::thread_common::*;
use crate::binding_util::*;
use crate::binding_util;
use flate2::read::ZlibDecoder;
use std::ffi::CString;
use std::{thread, sync::mpsc::*, path::Path};
use std::io::Read;
use rutie::*;
use std::{
    env,
    os::raw::{c_int, c_char}
};

pub static mut RGSS_RX : Option<Receiver<MessageTypes>> = None;

extern "C" {
    fn ruby_sysinit(argc: *mut c_int, argv: *mut *mut *mut c_char);
}

pub fn spawn_rgss_thread(rgss_rx: Receiver<MessageTypes>) -> thread::JoinHandle<()> {
    let thread = thread::spawn(move || {
        let sfml_tx = clone_sfml_tx(); // Shadow transmitters
        unsafe { RGSS_RX = Some(rgss_rx) }

        // Set up the VM
        unsafe {
            let args = env::args();
            let argv: Vec<String> = args.collect();
            let cstr_argv: Vec<_> = argv.iter()
                .map(|arg| CString::new(arg.as_str()).unwrap())
                .collect();
            let mut ptr_argv: Vec<_> = cstr_argv.iter()
                .map(|arg| arg.as_ptr() as *mut c_char)
                .collect();
            
            let mut cargc: i32 = ptr_argv.len() as i32;
            let mut cargv: *mut *mut c_char = ptr_argv.as_mut_ptr();
            ruby_sysinit(&mut cargc, &mut cargv);
        }
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
        let z_result = z.read_to_string(&mut s);
        match z_result {
            Err(why) => { println!("Zlib decoding of scripts failed with: {:?}", why) },
            Ok(_) => { }
        }

        // Eval script, and exit the thread if it errors
        let result = eval!(s, Binding::new(), name);
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