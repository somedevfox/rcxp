extern crate sdl2;

use sdl2::{
    Sdl,
    VideoSubsystem,
    video::Window
};
use std::process::{
    exit,
    Command
};

fn force_exit(message: String) {
    println!("{}", message);
    Command::new("pause").status();
    exit(1);
}

pub struct RCXPWindow {
    pub sdl_context: Sdl,
    pub sdl_window: Window
}

/*
* Maintainer's Notice:
* You might notice that there's an exit(1) after every force_exit function, yet force_exit
* Already does exit(1);
* Well... for some reason, if you remove exit(1) from every Err(why) => Rust compiler will immediately
* Compain about a variable being possibly-uninitialized.
* It's not needed for functionality, it's needed for *compilation*.
*/
pub fn create_window(title: &str, width: u32, height: u32) -> RCXPWindow {
    let ctx_attempt = sdl2::init();
    let ctx: Sdl;
    println!("Getting SDL2 Context...");
    match ctx_attempt {
        Err(why) => { force_exit(format!("Failed to initialize SDL2 Context. Error: {:?}", why)); exit(1) },
        Ok(sctx) => ctx = sctx
    }
    println!("Getting SDL2 Context... OK");

    let vs_attempt = ctx.video();
    let vs: VideoSubsystem;
    println!("Getting SDL2 Video Subsystem...");
    match vs_attempt {
        Err(why) => { force_exit(format!("Failed to get video subsystem. Error: {:?}", why)); exit(1) },
        Ok(svs) => vs = svs
    }
    println!("Getting SDL2 Video Subsystem... OK");

    let window_attempt = vs
        .window(title, width, height)
        .position_centered()
        .build();
    let window: Window;
    println!("Creating SDL2 Window...");
    match window_attempt {
        Err(why) => { force_exit(format!("Failed to create window. Error: {:?}", why)); exit(1) },
        Ok(swin) => window = swin
    }
    println!("Creating SDL2 Window... OK");

    println!("Successfully created window.");

    return RCXPWindow{
        sdl_context: ctx,
        sdl_window: window
    };
}
