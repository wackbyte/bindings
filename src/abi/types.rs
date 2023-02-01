//! FFI-safe types for interacting with Luau.

// TODO: maybe make FFI conversions on the Rust side
// E.g impl<T> ToForeign<RustVec<T>> for Vec<T>

/// FFI-safe vector received from bindings.
/// Capacity is guaranteed to be equal to length.
#[repr(C)]
pub struct RustVec<T> {
    content: *mut T,
    length: usize,
}

impl<T> From<Vec<T>> for RustVec<T> {
    fn from(mut vec: Vec<T>) -> RustVec<T> {
        // length == capacity
        vec.shrink_to_fit();

        let content = vec.as_mut_ptr();
        let length = vec.len();
        std::mem::forget(vec);

        RustVec { content, length }
    }
}

impl<T> From<RustVec<T>> for Vec<T> {
    fn from(string: RustVec<T>) -> Vec<T> {
        unsafe { Vec::from_raw_parts(string.content, string.length, string.length) }
    }
}

/// FFI-safe string received from bindings.
/// Capacity is guaranteed to be equal to length.
#[repr(C)]
pub struct RustString {
    content: *mut u8,
    length: usize,
}

impl From<String> for RustString {
    fn from(mut string: String) -> RustString {
        // length == capacity
        string.shrink_to_fit();

        let content = string.as_mut_ptr();
        let length = string.len();
        std::mem::forget(string);

        RustString { content, length }
    }
}

impl From<RustString> for String {
    fn from(string: RustString) -> String {
        unsafe { String::from_raw_parts(string.content, string.length, string.length) }
    }
}

/// FFI-safe option received from bindings.
#[repr(C)]
pub enum RustOption<T> {
    None,
    Some(T),
}

impl<T> From<Option<T>> for RustOption<T> {
    fn from(option: Option<T>) -> RustOption<T> {
        match option {
            Some(value) => RustOption::Some(value),
            None => RustOption::None,
        }
    }
}

impl<T> From<RustOption<T>> for Option<T> {
    fn from(option: RustOption<T>) -> Option<T> {
        match option {
            RustOption::Some(value) => Some(value),
            RustOption::None => None,
        }
    }
}

/// FFI-safe slice received from bindings.
#[repr(C)]
pub struct RustSlice<T> {
    content: *const T,
    length: usize,
}

impl<T> From<&[T]> for RustSlice<T> {
    fn from(slice: &[T]) -> RustSlice<T> {
        RustSlice {
            content: slice.as_ptr(),
            length: slice.len(),
        }
    }
}

impl From<&str> for RustSlice<u8> {
    fn from(slice: &str) -> RustSlice<u8> {
        RustSlice {
            content: slice.as_ptr(),
            length: slice.len(),
        }
    }
}
