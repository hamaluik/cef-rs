use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{cef_app_t, cef_base_ref_counted_t, cef_browser_process_handler_t};

use crate::application::Application;

#[ref_count]
#[derive(RefCount, Debug)]
#[allow(unused)]
pub struct App {
    pub app: cef_app_t,
    pub application: *mut Application,
}

impl App {
    pub fn allocate() -> *mut Self {
        let app = App {
            app: cef_app_t {
                base: cef_base_ref_counted_t {
                    size: size_of::<App>() as u64,
                    add_ref: Some(Self::add_ref),
                    release: Some(Self::release),
                    has_one_ref: Some(Self::has_one_ref),
                    has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                },
                on_before_command_line_processing: None,
                on_register_custom_schemes: None,
                get_render_process_handler: None,
                get_browser_process_handler: Some(Self::get_browser_process_handler),
                get_resource_bundle_handler: None,
            },
            application: std::ptr::null_mut(),
            ref_count: 1.into(),
        };

        log::debug!("app allocated");
        Box::into_raw(Box::from(app))
    }

    unsafe extern "C" fn get_browser_process_handler(
        slf: *mut cef_app_t,
    ) -> *mut cef_browser_process_handler_t {
        log::debug!(
            "getting browser process handler, slf: {:?}, application: {:?}",
            slf,
            *(*(slf as *mut App)).application
        );
        (*(*(slf as *mut App)).application).browser_process_handler_ptr()
    }
}
