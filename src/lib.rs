#![feature(is_some_with)]
#![feature(new_uninit)]

use std::{ffi::c_void, fs, path::Path};

use log::{error, info};
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HINSTANCE},
    System::{
        LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleA},
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        Threading::{CreateThread, THREAD_CREATION_FLAGS},
    },
};

use crate::host::SboxHost;

mod host;
mod utility;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(module: HINSTANCE, reason: u32, _reserved: *const u8) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            DisableThreadLibraryCalls(module);
            if let Ok(thread_handle) = CreateThread(
                std::ptr::null_mut() as *const _,
                0,
                Some(attach_thread),
                module.0 as *const _,
                THREAD_CREATION_FLAGS::default(),
                std::ptr::null_mut(),
            ) {
                CloseHandle(thread_handle);
            } else {
                return BOOL::from(false);
            }
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }

    BOOL::from(true)
}

unsafe extern "system" fn attach_thread(_module: *mut c_void) -> u32 {
    let sbox_host = utility::get_interface::<SboxHost>(
        GetModuleHandleA(win_pcstr!("engine2.dll")).unwrap(),
        "SboxHost001",
    )
    .expect("retrieve SBoxHost001 interface")
    .read();

    // Path to the TPA directory.
    let tpa_path = Path::new("./bin/managed/");

    // Write loader to TPA directory.
    info!("Writing loader to TPA...");
    let loader_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/Jackload.dll"));
    fs::write(tpa_path.join("Jackload.dll"), loader_bytes).expect("write loader to tpa");

    // Load loader.
    info!("Loading loader...");
    match sbox_host.load_assembly("Jackload", "Jackload.Loader", "Main") {
        Ok(_) => info!("Successfully loaded loader"),
        Err(e) => {
            error!("Failed to load loader with error {}", e)
        }
    };

    info!("All done!");

    1
}
