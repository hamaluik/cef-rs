use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{cef_base_ref_counted_t, cef_browser_t, cef_render_handler_t, cef_text_input_mode_t};

use crate::handler::Handler;

#[ref_count]
#[derive(RefCount)]
#[allow(unused)]
pub struct RenderHandler {
    pub render_handler: cef_render_handler_t,
    pub handler: *mut Handler,
}

impl RenderHandler {
    pub fn allocate() -> *mut Self {
        let render_handler = RenderHandler {
            render_handler: cef_render_handler_t {
                base: cef_base_ref_counted_t {
                    size: size_of::<RenderHandler>() as u64,
                    add_ref: Some(Self::add_ref),
                    release: Some(Self::release),
                    has_one_ref: Some(Self::has_one_ref),
                    has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                },
                get_accessibility_handler: None,
                get_root_screen_rect: None,
                get_view_rect: None,
                get_screen_point: None,
                get_screen_info: None,
                on_popup_show: None,
                on_popup_size: None,
                on_paint: None,
                on_accelerated_paint: None,
                start_dragging: None,
                update_drag_cursor: None,
                on_scroll_offset_changed: None,
                on_ime_composition_range_changed: None,
                on_text_selection_changed: None,
                on_virtual_keyboard_requested: Some(Self::on_virtual_keyboard_requested),
            },
            handler: std::ptr::null_mut(),
            ref_count: 1.into(),
        };

        log::debug!("render_handler allocated");
        Box::into_raw(Box::from(render_handler))
    }

    unsafe extern "C" fn on_virtual_keyboard_requested(
        _slf: *mut cef_render_handler_t,
        _browser: *mut cef_browser_t,
        input_mode: cef_text_input_mode_t,
    ) {
        log::debug!(
            "on virtual keyboard requested, input mode: {:?}",
            input_mode
        );
    }
}
