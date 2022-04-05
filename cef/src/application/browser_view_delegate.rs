use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{
    cef_base_ref_counted_t, cef_browser_view_delegate_t, cef_browser_view_t, cef_view_delegate_t,
    cef_window_create_top_level, cef_window_delegate_t,
};

use super::window_delegate::WindowDelegate;

#[ref_count]
#[derive(RefCount)]
#[allow(unused)]
pub struct BrowserViewDelegate {
    pub browser_view_delegate: cef_browser_view_delegate_t,
}

impl BrowserViewDelegate {
    pub fn allocate() -> *mut Self {
        let browser_view_delegate = BrowserViewDelegate {
            browser_view_delegate: cef_browser_view_delegate_t {
                base: cef_view_delegate_t {
                    base: cef_base_ref_counted_t {
                        size: size_of::<BrowserViewDelegate>() as u64,
                        add_ref: Some(Self::add_ref),
                        release: Some(Self::release),
                        has_one_ref: Some(Self::has_one_ref),
                        has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                    },
                    get_preferred_size: None,
                    get_minimum_size: None,
                    get_maximum_size: None,
                    get_height_for_width: None,
                    on_parent_view_changed: None,
                    on_child_view_changed: None,
                    on_window_changed: None,
                    on_layout_changed: None,
                    on_focus: None,
                    on_blur: None,
                },
                on_browser_created: None,
                on_browser_destroyed: None,
                get_delegate_for_popup_browser_view: None,
                on_popup_browser_view_created: Some(Self::on_popup_browser_view_created),
                get_chrome_toolbar_type: None,
            },
            ref_count: 1.into(),
        };

        log::debug!("browser_view_delegate allocated");
        Box::into_raw(Box::from(browser_view_delegate))
    }

    unsafe extern "C" fn on_popup_browser_view_created(
        _slf: *mut cef_browser_view_delegate_t,
        _browser_view: *mut cef_browser_view_t,
        popup_browser_view: *mut cef_browser_view_t,
        _is_dev_tools: i32,
    ) -> i32 {
        log::debug!(
            "on_popup_browser_view_created, opening top level window with view = {:?}",
            popup_browser_view
        );
        cef_window_create_top_level(
            WindowDelegate::allocate(popup_browser_view) as *mut cef_window_delegate_t
        );
        1
    }
}
