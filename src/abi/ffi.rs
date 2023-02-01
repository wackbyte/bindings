//! Bindings to Luau.

use crate::types::*;

use super::types::*;

include!(concat!(env!("OUT_DIR"), "/generated_bindings.rs"));
