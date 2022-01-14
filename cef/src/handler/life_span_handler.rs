use std::mem::size_of;

use cef_ref_counting::{ref_count, RefCount};
use cef_sys::{
    cef_base_ref_counted_t, cef_browser_t, cef_life_span_handler_t, cef_quit_message_loop,
};

use crate::handler::Handler;

#[ref_count]
#[derive(RefCount)]
#[allow(unused)]
pub struct LifeSpanHandler {
    pub life_span_handler: cef_life_span_handler_t,
    pub handler: *mut Handler,
}

impl LifeSpanHandler {
    pub fn allocate() -> *mut Self {
        let life_span_handler = LifeSpanHandler {
            life_span_handler: cef_life_span_handler_t {
                base: cef_base_ref_counted_t {
                    size: size_of::<LifeSpanHandler>() as u64,
                    add_ref: Some(Self::add_ref),
                    release: Some(Self::release),
                    has_one_ref: Some(Self::has_one_ref),
                    has_at_least_one_ref: Some(Self::has_at_least_one_ref),
                },
                on_before_popup: None,
                on_after_created: Some(Self::on_after_created),
                on_before_close: Some(Self::on_before_close),
                do_close: Some(Self::do_close),
            },
            handler: std::ptr::null_mut(),
            ref_count: 1.into(),
        };

        log::debug!("life_span_handler allocated");
        Box::into_raw(Box::from(life_span_handler))
    }

    unsafe extern "C" fn on_after_created(
        slf: *mut cef_life_span_handler_t,
        browser: *mut cef_browser_t,
    ) {
        log::debug!("on_after_created");
        (*((*(slf as *mut LifeSpanHandler)).handler))
            .browsers
            .push(browser);
    }

    unsafe extern "C" fn on_before_close(
        slf: *mut cef_life_span_handler_t,
        browser: *mut cef_browser_t,
    ) {
        log::debug!("on_before_close");
        for (i, b) in (*((*(slf as *mut LifeSpanHandler)).handler))
            .browsers
            .iter()
            .enumerate()
        {
            if (*browser).is_same.unwrap()(browser, *b) != 0 {
                log::debug!("removing browser {}", i);
                (*((*(slf as *mut LifeSpanHandler)).handler))
                    .browsers
                    .remove(i);
                break;
            }
        }

        if (*((*(slf as *mut LifeSpanHandler)).handler))
            .browsers
            .is_empty()
        {
            log::debug!("browser list is empty, quitting");
            cef_quit_message_loop();
        }
    }

    unsafe extern "C" fn do_close(
        slf: *mut cef_life_span_handler_t,
        _browser: *mut cef_browser_t,
    ) -> i32 {
        log::debug!("do_close");
        if (*((*(slf as *mut LifeSpanHandler)).handler)).browsers.len() == 1 {
            log::debug!("only 1 browser remaining, marking closing");
            (*((*(slf as *mut LifeSpanHandler)).handler)).is_closing = true;
        }
        0
    }
}
