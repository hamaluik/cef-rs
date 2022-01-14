use cef_sys::{cef_app_t, cef_base_ref_counted_t, cef_browser_process_handler_t};

mod app;
mod browser_process_handler;
mod browser_view_delegate;
mod window_delegate;

use app::App;
use browser_process_handler::BrowserProcessHandler;

#[derive(Debug)]
pub struct Application {
    app: *mut App,
    browser_process_handler: *mut BrowserProcessHandler,
}

impl Application {
    pub fn new() -> Application {
        let mut application = Application {
            app: App::allocate(),
            browser_process_handler: BrowserProcessHandler::allocate(),
        };
        application.apply_pointers();

        log::debug!("new application created: {:#?}", application);
        unsafe {
            log::trace!("  app -> {:#?}", (*application.app));
            log::trace!(
                "  browser_process_handler -> {:#?}",
                (*application.browser_process_handler)
            );
        }
        application
    }

    pub fn apply_pointers(&mut self) {
        unsafe {
            (*self.app).application = self;
            (*self.browser_process_handler).application = self;
        }
    }

    pub fn app_ptr(&self) -> *mut cef_app_t {
        log::trace!("add ref to app",);
        log::trace!("self: {:?}", self);
        unsafe {
            App::add_ref(self.app as *mut cef_base_ref_counted_t);
        }
        log::trace!("app -> {:p}", self.app);
        self.app as *mut cef_app_t
    }

    pub fn browser_process_handler_ptr(&self) -> *mut cef_browser_process_handler_t {
        log::trace!("add ref to browser_process_handler",);
        log::trace!("self: {:?}", self);
        unsafe {
            BrowserProcessHandler::add_ref(
                self.browser_process_handler as *mut cef_base_ref_counted_t,
            );
        }
        log::trace!(
            "browser_process_handler -> {:p}",
            self.browser_process_handler
        );
        self.browser_process_handler as *mut cef_browser_process_handler_t
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            log::debug!("dropping app");
            let app = self.app as *mut cef_base_ref_counted_t;
            while App::has_at_least_one_ref(app) == 1 {
                App::release(app);
            }
            log::debug!("dropping browser_process_handler");
            let browser_process_handler =
                self.browser_process_handler as *mut cef_base_ref_counted_t;
            while BrowserProcessHandler::has_at_least_one_ref(browser_process_handler) == 1 {
                BrowserProcessHandler::release(browser_process_handler);
            }
        }
    }
}
