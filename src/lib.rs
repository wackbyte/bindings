pub mod abi;
pub mod libraries;
pub mod mem;
pub mod types;

use std::panic;

pub use libraries::*;
pub use types::*;

extern "C" {
    fn clone_pointer(ptr: u32) -> u32;
    fn drop_pointer(ptr: u32);
}

#[no_mangle]
unsafe fn hook() {
    panic::set_hook(Box::new(|info| {
        let msg = info.to_string();
        error(&msg)
    }));
}
