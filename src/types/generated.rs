use crate::{
    abi::{ffi::*, types::*},
    clone_pointer, drop_pointer,
};

use super::{LuaValue, RbxScriptConnection};

include!(concat!(env!("OUT_DIR"), "/generated_types.rs"));
