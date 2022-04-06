use std::ptr::null_mut;

use anyhow::{anyhow, Result};
use application::Application;
use cef_sys::{
    cef_currently_on, cef_enable_highdpi_support, cef_execute_process, cef_initialize,
    cef_log_severity_t_LOGSEVERITY_WARNING, cef_main_args_t, cef_run_message_loop, cef_settings_t,
    cef_shutdown, cef_thread_id_t_TID_UI,
};

mod application;
mod cef_strings;
mod handler;

pub fn require_ui_thread() {
    unsafe {
        if cef_currently_on(cef_thread_id_t_TID_UI) == 0 {
            log::warn!("Not on UI thread!");
        }
    }
}

#[allow(unused)]
pub struct Cef {
    application: Application,
}

impl Cef {
    #[cfg(unix)]
    fn main_args() -> (Vec<std::ffi::CString>, cef_main_args_t) {
        use std::ffi::CString;
        use std::os::raw::{c_char, c_int};
        let args: Vec<CString> = std::env::args().map(|x| CString::new(x).unwrap()).collect();
        let _args: Vec<*mut c_char> = args.iter().map(|x| x.as_ptr() as *mut c_char).collect();
        (
            args,
            cef_main_args_t {
                argc: _args.len() as c_int,
                argv: _args.as_ptr() as *mut *mut c_char,
            },
        )
    }

    #[cfg(windows)]
    fn main_args() -> ((), cef_main_args_t) {
        (
            (),
            cef_main_args_t {
                instance: unsafe {
                    windows::Win32::System::LibraryLoader::GetModuleHandleA(windows::core::PCSTR(
                        null_mut(),
                    ))
                    .0 as *mut cef_sys::HINSTANCE__
                },
            },
        )
    }

    #[cfg(unix)]
    fn sandbox() -> *mut ::std::os::raw::c_void {
        null_mut()
    }

    #[cfg(windows)]
    fn sandbox() -> *mut ::std::os::raw::c_void {
        unsafe { cef_sys::cef_sandbox_info_create() }
    }

    pub fn initialize() -> Result<Cef> {
        unsafe {
            cef_enable_highdpi_support();
        }

        let sandbox_info = Self::sandbox();
        let (_args, main_args) = Self::main_args();

        // CEF applications have multiple sub-processes that share the same exe. Check for a
        // subprocess, and exit accordingly
        let exit_code = unsafe { cef_execute_process(&main_args, null_mut(), sandbox_info) };
        if exit_code >= 0 {
            std::process::exit(exit_code);
        }

        let settings = cef_settings_t {
            size: std::mem::size_of::<cef_settings_t>() as u64,
            no_sandbox: if cfg!(windows) { 0 } else { 1 },
            remote_debugging_port: 8765,
            command_line_args_disabled: 0,
            log_severity: cef_log_severity_t_LOGSEVERITY_WARNING,
            ..Default::default()
        };

        log::debug!("preparing application");
        let mut application = Application::new();
        application.apply_pointers();
        log::debug!("application prepared: {:?}", application);

        log::debug!("initializing");
        unsafe {
            if cef_initialize(&main_args, &settings, application.app_ptr(), sandbox_info) != 1 {
                return Err(anyhow!("failed to initialize!"));
            }
        }

        Ok(Cef { application })
    }

    pub fn run(mut self) -> Result<()> {
        self.application.apply_pointers();
        unsafe {
            log::debug!("running message loop");
            cef_run_message_loop();
            log::debug!("shutting down");
            cef_shutdown();
        }

        Ok(())
    }
}
