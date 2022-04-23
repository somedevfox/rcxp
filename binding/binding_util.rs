use rutie::*;

use std::sync::mpsc::*;

use crate::thread_common;

mod bitmap_binding;

pub fn bind_all() {
    bitmap_binding::bind();
}

pub fn marsh_load(obj: AnyObject) -> AnyObject {
    let marsh = Module::from_existing("Marshal");

    let result = marsh.protect_send("load", &[obj]);
    match result {
        Err(why) => { panic!("Marshal failed to load data: {:?}", why) },
        Ok(result) => { return result }
    }
}

pub fn anyobj_to_array(obj: AnyObject, err_str: &str) -> Array {
    let result = obj.try_convert_to::<Array>();
    match result {
        Ok(result) => { return result }
        Err(why) => { panic!("{} {:?}", err_str, why) }
    }
}

pub fn anyobj_to_rstring(obj: AnyObject, error_str: &str) -> RString {
    let result = obj.try_convert_to::<RString>();
    match result {
        Ok(result) => { return result }
        Err(why) => { panic!("{} {:?}", error_str, why) }
    }
}

pub fn anyobj_to_integer(obj: AnyObject, error_str: &str) -> Integer {
    let result = obj.try_convert_to::<Integer>();
    match result {
        Ok(result) => { return result }
        Err(why) => { panic!("{} {:?}", error_str, why) }
    }
}