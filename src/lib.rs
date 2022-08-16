#![feature(is_some_with)]
#![feature(new_uninit)]

use std::{ffi::c_void, fs, os::windows::prelude::AsRawHandle, path::Path};

use fern::colors::{Color, ColoredLevelConfig};
use log::{error, info};
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE, HINSTANCE},
    System::{
        Console::{AllocConsole, SetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE},
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
    if setup_console() {
        setup_logger();

        let sbox_host = utility::get_interface::<SboxHost>(
            GetModuleHandleA(win_pcstr!("engine2.dll")).unwrap(),
            "SboxHost001",
        )
        .expect("retrieve SBoxHost001 interface")
        .read();

        // Path to the TPA directory.
        let tpa_path = Path::new("./bin/managed/");
        // Path of our assemblies we want to load.
        let load_path = Path::new("./sideload/");
        // Create the path just in case this is first time using.
        fs::create_dir_all(load_path).expect("create load path dir");

        info!(
            "Looking in {} for assemblies to inject...",
            load_path.display()
        );
        info!("The entrypoint will be class 'ASSEMBLYNAME.Addon' method 'Main'");

        let paths = fs::read_dir(load_path).unwrap();

        // Find all assemblies to load and load them!
        paths.for_each(|path| {
            if let Ok(f) = path {
                let file_path = f.path();
                if file_path.extension().is_some_and(|&ext| ext.eq("dll")) {
                    let file_name = f.file_name();
                    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
                    info!("Loading assembly {:?}...", file_name);

                    fs::copy(&file_path, tpa_path.join(&file_name)).expect("copy to tpa dir");

                    match sbox_host.load_assembly(
                        file_stem,
                        format!("{}.{}", file_stem, "Addon").as_str(),
                        "Main",
                    ) {
                        Ok(_) => info!("Successfully loaded assembly {:?}", file_name),
                        Err(e) => {
                            error!("Failed to load assembly {:?} with error {}", file_name, e)
                        }
                    };
                }
            }
        });

        info!("All done!");

        1
    } else {
        0
    }
}

fn setup_logger() {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .trace(Color::BrightBlack);
    let colors_level = colors_line.info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%r"),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

// TODO: Switch to regular internal console
unsafe fn setup_console() -> bool {
    let allocated = AllocConsole().as_bool();

    if allocated {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open("CONOUT$")
            .unwrap();

        SetStdHandle(STD_OUTPUT_HANDLE, HANDLE(file.as_raw_handle() as _));
        SetStdHandle(STD_ERROR_HANDLE, HANDLE(file.as_raw_handle() as _));
        std::mem::forget(file);
    }

    allocated
}
