use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{
    cef_base_ref_counted_t, cef_client_t, cef_display_handler_t, cef_life_span_handler_t,
    cef_load_handler_t,
};

use crate::handler::Handler;

#[ref_count]
#[derive(RefCount)]
#[allow(unused)]
pub struct Client {
    pub client: cef_client_t,
    pub handler: *mut Handler,
}

impl Client {
    pub fn allocate() -> *mut Self {
        let client = Client {
            client: cef_client_t {
                base: cef_base_ref_counted_t {
                    size: size_of::<Client>() as u64,
                    add_ref: Some(Self::add_ref),
                    release: Some(Self::release),
                    has_one_ref: Some(Self::has_one_ref),
                    has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                },
                get_audio_handler: None,
                get_context_menu_handler: None,
                get_dialog_handler: None,
                get_display_handler: Some(Self::get_display_handler),
                get_download_handler: None,
                get_drag_handler: None,
                get_find_handler: None,
                get_focus_handler: None,
                get_frame_handler: None,
                get_jsdialog_handler: None,
                get_keyboard_handler: None,
                get_life_span_handler: Some(Self::get_life_span_handler),
                get_load_handler: Some(Self::get_load_handler),
                get_print_handler: None,
                get_render_handler: None,
                get_request_handler: None,
                on_process_message_received: None,
            },
            handler: std::ptr::null_mut(),
            ref_count: 1.into(),
        };

        log::debug!("client allocated");
        Box::into_raw(Box::from(client))
    }

    unsafe extern "C" fn get_display_handler(slf: *mut cef_client_t) -> *mut cef_display_handler_t {
        let slf = slf as *mut Client;
        log::debug!("getting display handler");
        (*((*slf).handler)).display_handler_ptr()
    }

    unsafe extern "C" fn get_life_span_handler(
        slf: *mut cef_client_t,
    ) -> *mut cef_life_span_handler_t {
        let slf = slf as *mut Client;
        log::debug!("getting life_span_handler");
        (*((*slf).handler)).life_span_handler_ptr()
    }

    unsafe extern "C" fn get_load_handler(slf: *mut cef_client_t) -> *mut cef_load_handler_t {
        let slf = slf as *mut Client;
        log::debug!("getting load_handler");
        (*((*slf).handler)).load_handler_ptr()
    }
}
