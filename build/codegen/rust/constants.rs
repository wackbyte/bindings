pub const ROBLOX_CREATABLE: &str = "\
pub trait RobloxCreatable {
	fn new() -> Self;
}

macro_rules! creatable {
	($($name:ident)*) => {
		$(
			impl RobloxCreatable for $name {
				fn new() -> $name {
					unsafe { std::mem::transmute(internal::instance_new(stringify!($name))) }
				}
			}
		)*
	}
}
";

pub const EXCLUSIVE_INSTANCE: &str = "\
macro_rules! impl_instance_exclusive {
	($name:ident) => {
		impl_instance!($name);

		impl std::convert::TryFrom<Instance> for $name {
			type Error = ();
			fn try_from(value: Instance) -> Result<Self, Self::Error> {
				if value.is_a(stringify!($name)) {
					unsafe { Ok(std::mem::transmute::<_, $name>(value)) }
				} else {
					Err(())
				}
			}
		}
	}
}
";

pub const RUST_INSTANCE_CUSTOM_IMPL: &str = "\
#[repr(transparent)]
pub struct $name(u32);

impl Clone for $name {
	fn clone(&self) -> Self {
		unsafe { Self(clone_pointer(self.to_ptr())) }
	}
}

impl Drop for $name {
	fn drop(&mut self) {
		unsafe { drop_pointer(self.to_ptr()) }
	}
}

impl From<$name> for LuaValue {
	fn from(value: $name) -> LuaValue {
		unsafe { std::mem::transmute::<_, LuaValue>(value) }
	}
}
";

pub const RUST_ROBLOX_ENUM_MACRO: &str = "\
macro_rules! roblox_enum {
    ($name:ident; { $($field:ident = $value:expr),*, }) => {
		#[allow(non_camel_case_types)]
		#[repr(C)]
        pub enum $name {
            $(
                $field = $value
            ),*
        }
    }
}";

pub const CUSTOM_IMPL_SERVICE: &str = "\
pub fn instance() -> Self {
	unsafe {
		std::mem::transmute::<_, Self>(DataModel::instance().get_service(stringify!($name)).unwrap())
	}
}";

pub const CUSTOM_IMPL_DATA_MODEL: &str = "\
pub fn instance() -> Self {
	extern \"C\" {
		fn get_game() -> u32;
	}

	Self(unsafe { get_game() })
}";

pub const CUSTOM_IMPL_INSTANCE: &str = "\
fn to_ptr(&self) -> u32 {
	self.0
}
pub fn downcast<I: From<$name>>(&self) -> I {
	self.clone().into()
}";

pub const DATATYPE_IMPL_MACRO: &str = "\
macro_rules! impl_data_type {
	($name:ident) => {
		#[repr(transparent)]
		pub struct $name(pub(super) u32);

		impl Drop for $name {
			fn drop(&mut self) {
				unsafe { drop_pointer(self.0) }
			}
		}

		impl From<$name> for LuaValue {
			fn from(value: $name) -> LuaValue {
				unsafe { std::mem::transmute::<_, LuaValue>(value) }
			}
		}
	}
}";
