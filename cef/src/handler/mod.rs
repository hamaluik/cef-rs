use cef_sys::{
    cef_base_ref_counted_t, cef_browser_t, cef_client_t, cef_display_handler_t,
    cef_life_span_handler_t, cef_load_handler_t,
};

use self::{
    client::Client, display_handler::DisplayHandler, life_span_handler::LifeSpanHandler,
    load_handler::LoadHandler,
};

mod client;
mod display_handler;
mod life_span_handler;
mod load_handler;

#[derive(Debug)]
pub struct Handler {
    client: *mut Client,
    display_handler: *mut DisplayHandler,
    life_span_handler: *mut LifeSpanHandler,
    load_handler: *mut LoadHandler,
    browsers: Vec<*mut cef_browser_t>,
    is_closing: bool,
}

impl Handler {
    pub fn new() -> Handler {
        let mut handler = Handler {
            client: Client::allocate(),
            display_handler: DisplayHandler::allocate(),
            life_span_handler: LifeSpanHandler::allocate(),
            load_handler: LoadHandler::allocate(),
            browsers: Vec::new(),
            is_closing: false,
        };
        handler.apply_pointers();

        log::debug!("handler created");
        handler
    }

    pub fn apply_pointers(&mut self) {
        unsafe {
            (*self.client).handler = self;
            (*self.display_handler).handler = self;
            (*self.life_span_handler).handler = self;
            (*self.load_handler).handler = self;
        }
    }

    pub fn client_ptr(&self) -> *mut cef_client_t {
        unsafe {
            Client::add_ref(self.client as *mut cef_base_ref_counted_t);
        }
        self.client as *mut cef_client_t
    }

    pub fn display_handler_ptr(&self) -> *mut cef_display_handler_t {
        unsafe {
            DisplayHandler::add_ref(self.display_handler as *mut cef_base_ref_counted_t);
        }
        self.display_handler as *mut cef_display_handler_t
    }

    pub fn life_span_handler_ptr(&self) -> *mut cef_life_span_handler_t {
        unsafe {
            LifeSpanHandler::add_ref(self.life_span_handler as *mut cef_base_ref_counted_t);
        }
        self.life_span_handler as *mut cef_life_span_handler_t
    }

    pub fn load_handler_ptr(&self) -> *mut cef_load_handler_t {
        unsafe {
            LoadHandler::add_ref(self.load_handler as *mut cef_base_ref_counted_t);
        }
        self.load_handler as *mut cef_load_handler_t
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        unsafe {
            log::debug!("dropping client");
            let client = self.client as *mut cef_base_ref_counted_t;
            while Client::has_at_least_one_ref(client) == 1 {
                Client::release(client);
            }

            log::debug!("dropping display_handler");
            let display_handler = self.display_handler as *mut cef_base_ref_counted_t;
            while DisplayHandler::has_at_least_one_ref(display_handler) == 1 {
                DisplayHandler::release(display_handler);
            }

            log::debug!("dropping life_span_handler");
            let life_span_handler = self.life_span_handler as *mut cef_base_ref_counted_t;
            while LifeSpanHandler::has_at_least_one_ref(life_span_handler) == 1 {
                LifeSpanHandler::release(life_span_handler);
            }

            log::debug!("dropping load_handler");
            let load_handler = self.load_handler as *mut cef_base_ref_counted_t;
            while LoadHandler::has_at_least_one_ref(load_handler) == 1 {
                LoadHandler::release(load_handler);
            }
        }
    }
}
