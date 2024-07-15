use once_cell::sync::Lazy;
use std::{collections::HashMap, ffi::c_int, sync::RwLock};

#[repr(C)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

type Id = c_int;

#[no_mangle]
pub extern "C" fn get_force(instance_id: Id, t: f64, force: &mut Vec2) {
    *force = match HANDLERS
        .read()
        .expect("other threads not to panic")
        .get(&instance_id)
    {
        Some(handler) => handler(t),
        None => Vec2 { x: 0.0, y: 0.0 },
    };
}

static HANDLERS: Lazy<RwLock<HashMap<Id, Handler>>> = Lazy::new(|| RwLock::new(HashMap::new()));

type Handler = extern "C" fn(f64) -> Vec2;

#[no_mangle]
pub extern "C" fn register_handler(instance_id: Id, handler: Handler) {
    HANDLERS
        .write()
        .expect("other threads not to panic")
        .insert(instance_id, handler);
}

pub type RegisterHandlerFn = extern "C" fn(Id, Handler);
