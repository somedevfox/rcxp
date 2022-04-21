mod sfml_thread;

use crate::sfml_thread::*;

fn main() {
    let mut sfml_thread = SFMLThread::new(640, 480, String::from("Test"));
    while sfml_thread.window_open() {
        sfml_thread.update()
    }
}
