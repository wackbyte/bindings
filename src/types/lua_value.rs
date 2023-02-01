use std::convert::TryFrom;

use crate::{abi::ffi::internal, drop_pointer};

#[repr(transparent)]
pub struct LuaValue(pub(crate) u32);

impl Drop for LuaValue {
    fn drop(&mut self) {
        unsafe { drop_pointer(self.0) }
    }
}

impl From<&str> for LuaValue {
    fn from(value: &str) -> LuaValue {
        internal::string_to_lua_value(value)
    }
}

impl TryFrom<LuaValue> for String {
    type Error = ();

    fn try_from(value: LuaValue) -> Result<String, Self::Error> {
        internal::lua_value_to_string(&value).ok_or(())
    }
}

impl TryFrom<LuaValue> for f64 {
    type Error = ();

    fn try_from(value: LuaValue) -> Result<f64, Self::Error> {
        internal::lua_value_to_float(&value).ok_or(())
    }
}

macro_rules! impl_scalar_lua_value {
	($($name:ident($type:ident))*) => {
		$(
            extern "C" {
                fn $name(value: $type) -> u32;
            }

			impl From<$type> for LuaValue {
				fn from(value: $type) -> LuaValue {
					unsafe { Self($name(value)) }
				}
			}
		)*
	}
}

impl_scalar_lua_value!(
    lua_value_f32(f32)
    lua_value_f64(f64)

    lua_value_i8(i8)
    lua_value_i16(i16)
    lua_value_i32(i32)
    lua_value_i64(i64)

    lua_value_u8(u8)
    lua_value_u16(u16)
    lua_value_u32(u32)
    lua_value_u64(u64)

    lua_value_usize(usize)
    lua_value_isize(isize)
);
