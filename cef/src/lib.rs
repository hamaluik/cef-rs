use std::ptr::null_mut;

use anyhow::{anyhow, Result};
use application::Application;
use cef_sys::{
    cef_execute_process, cef_initialize, cef_log_severity_t_LOGSEVERITY_INFO, cef_main_args_t,
    cef_run_message_loop, cef_settings_t, cef_shutdown,
};

mod application;
mod cef_strings;
mod handler;

#[allow(unused)]
pub struct Cef {
    application: Application,
}

impl Cef {
    pub fn initialize() -> Result<Cef> {
        use std::ffi::CString;
        use std::os::raw::{c_char, c_int};
        let args: Vec<CString> = std::env::args().map(|x| CString::new(x).unwrap()).collect();
        let args: Vec<*mut c_char> = args.iter().map(|x| x.as_ptr() as *mut c_char).collect();
        let main_args = cef_main_args_t {
            argc: args.len() as c_int,
            argv: args.as_ptr() as *mut *mut c_char,
        };

        // CEF applications have multiple sub-processes that share the same exe. Check for a
        // subprocess, and exit accordingly
        let exit_code = unsafe { cef_execute_process(&main_args, null_mut(), null_mut()) };
        if exit_code >= 0 {
            std::process::exit(exit_code);
        }

        let settings = cef_settings_t {
            size: std::mem::size_of::<cef_settings_t>() as u64,
            no_sandbox: 1,
            remote_debugging_port: 1721,
            command_line_args_disabled: 1,
            log_severity: cef_log_severity_t_LOGSEVERITY_INFO,
            ..Default::default()
        };

        log::debug!("preparing application");
        let mut application = Application::new();
        application.apply_pointers();
        log::debug!("application prepared: {:?}", application);

        log::debug!("initializing");
        unsafe {
            if cef_initialize(&main_args, &settings, application.app_ptr(), null_mut()) != 1 {
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
