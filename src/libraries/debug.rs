use crate::abi::types::{RustSlice, RustString};

extern "C" {
    fn lib_debug_traceback() -> RustString;
    fn lib_debug_profilebegin(label: RustSlice<u8>);
    fn lib_debug_profileend();
    fn lib_debug_setmemorycategory(tag: RustSlice<u8>);
    fn lib_debug_resetmemorycategory();
}

pub fn traceback() -> String {
    unsafe { lib_debug_traceback() }.into()
}

pub fn profile_begin(label: &str) {
    unsafe {
        lib_debug_profilebegin(RustSlice::from(label.as_bytes()));
    }
}

pub fn profile_end() {
    unsafe {
        lib_debug_profileend();
    }
}

pub fn set_memory_category(tag: &str) {
    unsafe {
        lib_debug_setmemorycategory(RustSlice::from(tag.as_bytes()));
    }
}

pub fn reset_memory_category() {
    unsafe {
        lib_debug_resetmemorycategory();
    }
}
