use std::{
    ffi::{c_char, c_void},
    mem::transmute,
};

use windows::Win32::{
    Foundation::{FARPROC, HINSTANCE},
    System::LibraryLoader::GetProcAddress,
};

#[macro_export]
macro_rules! c_str {
    ($string:expr) => {
        concat!($string, "\0").as_ptr() as *const core::ffi::c_char
    };
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\0"), $($arg)*).as_ptr() as *const core::ffi::c_char);
}

#[macro_export]
macro_rules! win_pcstr {
    ($string:expr) => {
        windows::core::PCSTR(concat!($string, "\0").as_ptr())
    };
    ($fmt:expr, $($arg:tt)*) => (windows::core::PCSTR(format!(concat!($fmt, "\0"), $($arg)*).as_ptr()));
}

pub unsafe fn get_interface<T: Sized>(module: HINSTANCE, name: &str) -> Option<*mut T> {
    // Get create_interface from the specified module. (i.e. client.dll)
    let create_interface_ptr: FARPROC = GetProcAddress(module, win_pcstr!("CreateInterface"));
    let create_interface = transmute::<
        _,
        unsafe extern "C" fn(name: *const c_char, return_code: i8) -> *const c_void,
    >(create_interface_ptr.unwrap());

    // Retrieve interface using `CreateInterface`.
    let interface = create_interface(c_str!("{}", name), 0);

    if interface.is_null() {
        None
    } else {
        Some(interface as *mut T)
    }
}
