use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{
    cef_base_ref_counted_t, cef_browser_t, cef_browser_view_get_for_browser, cef_display_handler_t,
    cef_string_t,
};

use crate::handler::Handler;

#[ref_count]
#[derive(RefCount)]
#[allow(unused)]
pub struct DisplayHandler {
    pub display_handler: cef_display_handler_t,
    pub handler: *mut Handler,
}

impl DisplayHandler {
    pub fn allocate() -> *mut Self {
        let display_handler = DisplayHandler {
            display_handler: cef_display_handler_t {
                base: cef_base_ref_counted_t {
                    size: size_of::<DisplayHandler>() as u64,
                    add_ref: Some(Self::add_ref),
                    release: Some(Self::release),
                    has_one_ref: Some(Self::has_one_ref),
                    has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                },
                on_title_change: Some(Self::on_title_change),
                on_tooltip: None,
                on_auto_resize: None,
                on_cursor_change: None,
                on_address_change: None,
                on_status_message: None,
                on_console_message: None,
                on_favicon_urlchange: None,
                on_fullscreen_mode_change: None,
                on_loading_progress_change: None,
            },
            handler: std::ptr::null_mut(),
            ref_count: 1.into(),
        };

        log::debug!("display_handler allocated");
        Box::into_raw(Box::from(display_handler))
    }

    unsafe extern "C" fn on_title_change(
        _slf: *mut cef_display_handler_t,
        browser: *mut cef_browser_t,
        title: *const cef_string_t,
    ) {
        log::debug!("on_title_change");
        let browser_view = cef_browser_view_get_for_browser(browser);
        if browser_view as usize != 0 {
            let window = (*browser_view).base.get_window.unwrap()(&mut (*browser_view).base);
            if window as usize != 0 {
                (*window).set_title.unwrap()(window, title);
            }
        }
    }
}
