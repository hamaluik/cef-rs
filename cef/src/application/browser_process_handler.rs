use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{
    cef_base_ref_counted_t, cef_browser_process_handler_t, cef_browser_settings_t,
    cef_browser_view_create, cef_browser_view_delegate_t, cef_client_t,
    cef_window_create_top_level, cef_window_delegate_t,
};

use crate::{
    application::{browser_view_delegate::BrowserViewDelegate, window_delegate::WindowDelegate},
    cef_strings::to_cef_str,
    handler::Handler,
};

use super::Application;

#[ref_count]
#[derive(RefCount, Debug)]
#[allow(unused)]
pub struct BrowserProcessHandler {
    pub browser_process_handler: cef_browser_process_handler_t,
    pub application: *mut Application,
    pub handler: Option<Handler>,
}

impl BrowserProcessHandler {
    pub fn allocate() -> *mut Self {
        let browser_process_handler = BrowserProcessHandler {
            browser_process_handler: cef_browser_process_handler_t {
                base: cef_base_ref_counted_t {
                    size: size_of::<BrowserProcessHandler>() as u64,
                    add_ref: Some(Self::add_ref),
                    release: Some(Self::release),
                    has_one_ref: Some(Self::has_one_ref),
                    has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                },
                on_context_initialized: Some(Self::on_context_initialized),
                get_default_client: Some(Self::get_default_client),
                on_before_child_process_launch: None,
                on_schedule_message_pump_work: None,
            },
            application: std::ptr::null_mut(),
            handler: None,
            ref_count: 1.into(),
        };

        log::debug!(
            "browser_process_handler allocated: {:?}",
            browser_process_handler
        );
        Box::into_raw(Box::from(browser_process_handler))
    }

    unsafe extern "C" fn on_context_initialized(slf: *mut cef_browser_process_handler_t) {
        log::debug!("on_context_initialized");
        let url = to_cef_str("https://google.ca");
        let settings = cef_browser_settings_t::default();

        let mut handler = Handler::new();
        handler.apply_pointers();
        log::debug!("creating browser_view");
        let browser_view = cef_browser_view_create(
            handler.client_ptr(),
            &url,
            &settings,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            BrowserViewDelegate::allocate() as *mut cef_browser_view_delegate_t,
        );

        log::debug!("creating top level window");
        cef_window_create_top_level(
            WindowDelegate::allocate(browser_view) as *mut cef_window_delegate_t
        );

        (*(slf as *mut BrowserProcessHandler)).handler = Some(handler);
        (*(slf as *mut BrowserProcessHandler))
            .handler
            .as_mut()
            .unwrap()
            .apply_pointers();
    }

    unsafe extern "C" fn get_default_client(
        slf: *mut cef_browser_process_handler_t,
    ) -> *mut cef_client_t {
        let slf = slf as *mut BrowserProcessHandler;
        if let Some(handler) = &(*(slf as *mut BrowserProcessHandler)).handler {
            log::debug!("getting default client: {:p}", handler.client_ptr());
            handler.client_ptr()
        } else {
            log::debug!("getting default client but NULL");
            std::ptr::null_mut()
        }
    }
}
