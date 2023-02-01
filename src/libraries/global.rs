use crate::abi::types::{RustSlice, RustString};

extern "C" {
    fn global_lua_version() -> RustString;
    fn global_roblox_version() -> RustString;

    fn global_print(msg: RustSlice<u8>);
    fn global_warn(msg: RustSlice<u8>);
    fn global_error(msg: RustSlice<u8>) -> !;
    fn global_type(ptr: u32) -> RustString;
    fn global_typeof(ptr: u32) -> RustString;
    fn global_tick() -> f64;
    fn global_time() -> f64;
    fn global_elapsed_time() -> f64;
}

// TODO: consider using lazy_static/a oncecell for these two?
pub fn lua_version() -> String {
    unsafe { global_lua_version() }.into()
}

pub fn roblox_version() -> String {
    unsafe { global_roblox_version() }.into()
}

pub fn tick() -> f64 {
    unsafe { global_tick() }
}

pub fn time() -> f64 {
    unsafe { global_time() }
}

pub fn elapsed_time() -> f64 {
    unsafe { global_elapsed_time() }
}

pub fn print(msg: &str) {
    unsafe {
        global_print(RustSlice::from(msg));
    }
}

pub fn warn(msg: &str) {
    unsafe {
        global_warn(RustSlice::from(msg));
    }
}

/// For use in a panic hook.
pub fn error(msg: &str) -> ! {
    unsafe { global_error(RustSlice::from(msg)) }
}

#[macro_export]
macro_rules! print {
    ($($tts:tt)*) => {{
        let msg = format!($($tts)*);
        $crate::libraries::print(&msg)
    }};
}

#[macro_export]
macro_rules! warn {
    ($($tts:tt)*) => {{
        let msg = format!($($tts)*);
        $crate::libraries::warn(&msg)
    }};
}

#[macro_export]
macro_rules! error {
    ($($tts:tt)*) => {{
        let msg = format!($($tts)*);
        $crate::libraries::error(&msg)
    }};
}
