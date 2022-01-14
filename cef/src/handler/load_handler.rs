use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{
    cef_base_ref_counted_t, cef_browser_t, cef_errorcode_t_ERR_ABORTED, cef_frame_t,
    cef_load_handler_t, cef_string_t,
};

use crate::{
    cef_strings::{from_cef_str, to_cef_str},
    handler::Handler,
};

#[ref_count]
#[derive(RefCount)]
#[allow(unused)]
pub struct LoadHandler {
    pub load_handler: cef_load_handler_t,
    pub handler: *mut Handler,
}

impl LoadHandler {
    pub fn allocate() -> *mut Self {
        let load_handler = LoadHandler {
            load_handler: cef_load_handler_t {
                base: cef_base_ref_counted_t {
                    size: size_of::<LoadHandler>() as u64,
                    add_ref: Some(Self::add_ref),
                    release: Some(Self::release),
                    has_one_ref: Some(Self::has_one_ref),
                    has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                },
                on_loading_state_change: None,
                on_load_end: None,
                on_load_error: Some(Self::on_load_error),
                on_load_start: None,
            },
            handler: std::ptr::null_mut(),
            ref_count: 1.into(),
        };

        log::debug!("load_handler allocated");
        Box::into_raw(Box::from(load_handler))
    }

    unsafe extern "C" fn on_load_error(
        slf: *mut cef_load_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        error_code: i32,
        error_text: *const cef_string_t,
        failed_url: *const cef_string_t,
    ) {
        log::debug!("on_load_error");
        // don't display an error for downloaded files
        if error_code == cef_errorcode_t_ERR_ABORTED {
            return;
        }

        let error_text = from_cef_str(error_text);
        let failed_url = from_cef_str(failed_url);

        let html =format!("<html><body bgcolor='white'><h2>Failed to load!</h2><p>Failed to load URL '{}' with error: {} ({}).</p></body></html>", failed_url, error_text, error_code);
        let html = to_cef_str(html);

        // todo!()
    }
}
