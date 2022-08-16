use std::{ffi::c_void, mem::transmute};

use log::info;

use crate::c_str;

/// This is SboxHost001 appsystem interface in engine2.dll.
#[derive(Debug)]
#[repr(C)]
pub struct SboxHost {
    pub vtable: *mut SboxHostVtable,
    pad_0x0: [u8; 0x20],
    pub init: fns::Initialize,
    pub shutdown: fns::Shutdown,
    pub create_delegate: fns::CreateDelegate,
    pub handle: u64,
    pub domain_id: u64,
}

impl SboxHost {
    pub unsafe fn load_assembly(
        &self,
        name: &str,
        class: &str,
        method: &str,
    ) -> Result<(), windows::core::Error> {
        let mut fun_boxed = Box::<unsafe extern "C" fn()>::new_uninit();

        let hr = (self.create_delegate)(
            self.handle as *mut c_void,
            self.domain_id,
            c_str!("{}", name),
            c_str!("{}", class),
            c_str!("{}", method),
            fun_boxed.as_mut_ptr() as *mut usize,
        );

        if hr.is_ok() {
            // Call delegate.
            // TODO: Add configuration for params and return type.
            let fun = fun_boxed.assume_init_read();
            info!("delegate: {:?}", fun);
            fun();
        }

        hr.ok()
    }
}

#[repr(C)]
pub struct SboxHostVtable {
    pad_0x0: [u8; 0x18],
    pub connect: *mut fns::Initialize,
}

#[allow(unused)]
mod fns {
    #[cfg(all(windows, target_arch = "x86"))]
    pub use x86::*;

    #[cfg(all(windows, target_arch = "x86_64"))]
    pub use x64::*;

    mod x64 {
        use std::ffi::c_void;
        use std::os::raw::c_char;

        use windows::core::HRESULT;

        /// Initialize the CoreCLR. Creates and starts CoreCLR host and creates an app domain
        ///
        /// # Arguments
        ///
        /// * `exe_path` - An absolute path to the executable that invoked the ExecuteAssembly (the native host application)
        /// * `appdomain_name` - The friendly name of the app domain that will be created to execute the assembly
        /// * `property_count` - The number of properties (elements of the following two arguments)
        /// * `property_keys` - An array of keys to properties of the app domain
        /// * `property_values` - An array of values to properties of the app domain
        /// * `host_handle` - Output parameter, handle of the created host
        /// * `domain_id` - Output parameter, id of the created app domain
        pub type Initialize = unsafe extern "C" fn(
            exe_path: *const c_char,
            appdomain_name: *const c_char,
            property_count: i64,
            property_keys: *const *const c_char,
            property_values: *const *const c_char,
            host_handle: *mut *mut c_void,
            domain_id: *mut u64,
        ) -> i64;

        /// Shutdown CoreCLR. It unloads the app domain and stops the CoreCLR host
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        pub type Shutdown = unsafe extern "C" fn(host_handle: *mut c_void, domain_id: u64) -> i64;

        /// Shutdown CoreCLR. It unloads the app domain and stops the CoreCLR host
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        /// * `exit_code` - The latched exit code after the domain is unloaded
        pub type ShutdownWithCode = unsafe extern "C" fn(
            host_handle: *mut c_void,
            domain_id: u64,
            exit_code: *mut i64,
        ) -> i64;

        /// Create a native callable function pointer for a managed method
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        /// * `entry_point_assembly` - The name of the assembly which holds the custom entry point
        /// * `entry_point_type` - The name of the type which holds the custom entry point
        /// * `entry_point_method` - The name of the method which is the custom entry point
        /// * `delegate` - The function pointer to be filled by the function call to the entry point
        pub type CreateDelegate = unsafe extern "C" fn(
            host_handle: *mut c_void,
            domain_id: u64,
            entry_point_assembly: *const c_char,
            entry_point_type: *const c_char,
            entry_point_method: *const c_char,
            delegate: *mut usize,
        ) -> HRESULT;

        /// Execute a managed assembly with given arguments
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        /// * `arg_count` - The number of arguments passed to the executed assembly
        /// * `arg_values` - An array of arguments passed to the executed assembly
        /// * `entry_point_method` - The path of the managed assembly to execute (or NULL if using a custom entrypoint).
        /// * `exit_code` - The exit code returned by the executed assembly
        pub type ExecuteAssembly = unsafe extern "C" fn(
            host_handle: *mut c_void,
            domain_id: u64,
            arg_count: i64,
            arg_values: *const *const c_char,
            managed_assembly_path: *const c_char,
            exit_code: *mut u64,
        ) -> HRESULT;
    }

    mod x86 {
        use std::ffi::c_void;
        use std::os::raw::c_char;

        use windows::core::HRESULT;

        /// Initialize the CoreCLR. Creates and starts CoreCLR host and creates an app domain
        ///
        /// # Arguments
        ///
        /// * `exe_path` - An absolute path to the executable that invoked the ExecuteAssembly (the native host application)
        /// * `appdomain_name` - The friendly name of the app domain that will be created to execute the assembly
        /// * `property_count` - The number of properties (elements of the following two arguments)
        /// * `property_keys` - An array of keys to properties of the app domain
        /// * `property_values` - An array of values to properties of the app domain
        /// * `host_handle` - Output parameter, handle of the created host
        /// * `domain_id` - Output parameter, id of the created app domain
        pub type Initialize = unsafe extern "stdcall" fn(
            exe_path: *const c_char,
            appdomain_name: *const c_char,
            property_count: i32,
            property_keys: *const *const c_char,
            property_values: *const *const c_char,
            host_handle: *mut *mut c_void,
            domain_id: *mut u32,
        ) -> i32;

        /// Shutdown CoreCLR. It unloads the app domain and stops the CoreCLR host
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        pub type Shutdown =
            unsafe extern "stdcall" fn(host_handle: *mut c_void, domain_id: u32) -> i32;

        /// Shutdown CoreCLR. It unloads the app domain and stops the CoreCLR host
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        /// * `exit_code` - The latched exit code after the domain is unloaded
        pub type ShutdownWithCode = unsafe extern "stdcall" fn(
            host_handle: *mut c_void,
            domain_id: u32,
            exit_code: *mut i32,
        ) -> i32;

        /// Create a native callable function pointer for a managed method
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        /// * `entry_point_assembly` - The name of the assembly which holds the custom entry point
        /// * `entry_point_type` - The name of the type which holds the custom entry point
        /// * `entry_point_method` - The name of the method which is the custom entry point
        pub type CreateDelegate = unsafe extern "stdcall" fn(
            host_handle: *mut c_void,
            domain_id: u32,
            entry_point_assembly: *const c_char,
            entry_point_type: *const c_char,
            entry_point_method: *const c_char,
        ) -> HRESULT;

        /// Execute a managed assembly with given arguments
        ///
        /// # Arguments
        ///
        /// * `host_handle` - A handle to the host
        /// * `domain_id` - The id of the domain
        /// * `arg_count` - The number of arguments passed to the executed assembly
        /// * `arg_values` - An array of arguments passed to the executed assembly
        /// * `entry_point_method` - The path of the managed assembly to execute (or NULL if using a custom entrypoint).
        /// * `exit_code` - The exit code returned by the executed assembly
        pub type ExecuteAssembly = unsafe extern "stdcall" fn(
            host_handle: *mut c_void,
            domain_id: u32,
            arg_count: i32,
            arg_values: *const *const c_char,
            managed_assembly_path: *const c_char,
            exit_code: *mut u32,
        ) -> HRESULT;
    }
}
