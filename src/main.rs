mod sfml_thread;
mod thread_common;
mod rgss_thread;

use crate::sfml_thread::*;
use crate::rgss_thread::*;

fn main() {
    let mut sfml_thread = SFMLThread::new(640, 480, String::from("Test"));
    let mut rgss_thread = RGSSThread::new();
    while sfml_thread.window_open() {
        sfml_thread.update();
        rgss_thread.update();
    }
}
