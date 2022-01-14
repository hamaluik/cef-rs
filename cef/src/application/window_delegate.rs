use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{
    cef_base_ref_counted_t, cef_browser_view_t, cef_panel_delegate_t, cef_size_t,
    cef_view_delegate_t, cef_view_t, cef_window_delegate_t, cef_window_t,
};

#[ref_count]
#[derive(RefCount)]
#[allow(unused)]
pub struct WindowDelegate {
    pub window_delegate: cef_window_delegate_t,
    browser_view: *mut cef_browser_view_t,
}

impl WindowDelegate {
    pub fn allocate(browser_view: *mut cef_browser_view_t) -> *mut Self {
        let window_delegate = WindowDelegate {
            window_delegate: cef_window_delegate_t {
                base: cef_panel_delegate_t {
                    base: cef_view_delegate_t {
                        base: cef_base_ref_counted_t {
                            size: std::mem::size_of::<WindowDelegate>() as u64,
                            add_ref: Some(Self::add_ref),
                            release: Some(Self::release),
                            has_one_ref: Some(Self::has_one_ref),
                            has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                        },
                        get_preferred_size: Some(Self::get_preferred_size),
                        get_minimum_size: None,
                        get_maximum_size: None,
                        get_height_for_width: None,
                        on_parent_view_changed: None,
                        on_child_view_changed: None,
                        on_focus: None,
                        on_blur: None,
                        on_window_changed: None,
                        on_layout_changed: None,
                    },
                },
                on_window_created: Some(Self::on_window_created),
                on_window_destroyed: Some(Self::on_window_destroyed),
                get_parent_window: None,
                is_frameless: None,
                can_resize: None,
                can_maximize: None,
                can_minimize: None,
                can_close: Some(Self::can_close),
                on_accelerator: None,
                on_key_event: None,
                get_initial_bounds: None,
                get_initial_show_state: None,
            },
            browser_view,
            ref_count: 1.into(),
        };

        log::debug!("window_delegate allocated");
        Box::into_raw(Box::from(window_delegate))
    }

    unsafe extern "C" fn on_window_created(
        slf: *mut cef_window_delegate_t,
        window: *mut cef_window_t,
    ) {
        log::debug!("on_window_created, adding child view");
        let slf = slf as *mut WindowDelegate;
        ((*window).base).add_child_view.unwrap()(
            &mut (*window).base,
            &mut (*((*slf).browser_view)).base,
        );
        log::debug!("showing window");
        (*window).show.unwrap()(window);

        //log::debug!("focussing window");
        //(*((*slf).browser_view)).base.request_focus.unwrap()(&mut (*((*slf).browser_view)).base);
    }

    unsafe extern "C" fn on_window_destroyed(
        slf: *mut cef_window_delegate_t,
        _window: *mut cef_window_t,
    ) {
        log::debug!("on_window_destroyed");
        (*(slf as *mut WindowDelegate)).browser_view = std::ptr::null_mut();
    }

    unsafe extern "C" fn get_preferred_size(
        _: *mut cef_view_delegate_t,
        _view: *mut cef_view_t,
    ) -> cef_size_t {
        log::debug!("getting preferred size (800x600)");
        cef_size_t {
            width: 800,
            height: 600,
        }
    }

    unsafe extern "C" fn can_close(
        slf: *mut cef_window_delegate_t,
        _window: *mut cef_window_t,
    ) -> i32 {
        log::debug!("can_close");
        let get_browser = (*(*(slf as *mut WindowDelegate)).browser_view).get_browser;
        if let Some(get_browser) = get_browser {
            let browser = get_browser((*(slf as *mut WindowDelegate)).browser_view);
            if browser as usize != 0 {
                log::debug!("trying to close browser");
                let host = (*browser).get_host.unwrap()(browser);
                (*host).try_close_browser.unwrap()(host);
            } else {
                log::debug!("can't close browser, there isn't one");
            }
        }
        1
    }
}
