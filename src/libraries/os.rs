extern "C" {
    fn lib_os_clock() -> f64;
}

pub fn clock() -> f64 {
    unsafe { lib_os_clock() }
}
