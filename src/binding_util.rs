use rutie::*;

pub fn marsh_load(string: String) -> AnyObject {
    let rstring = RString::from(string);
    let marsh = Module::from_existing("Marshal");

    let result = unsafe { marsh.protect_send("load", &[AnyObject::from(&rstring)]) };
    match result {
        Err(why) => { panic!("Marshal failed to load data: {:?}", why) },
        Ok(result) => { return result }
    }
}

pub fn anyobj_to_array(obj: AnyObject, err_str: String) -> Array {
    let result = obj.try_convert_to::<Array>();
    match result {
        Ok(result) => { return result }
        Err(why) => { panic!("{} {:?}", err_str, why) }
    }
}