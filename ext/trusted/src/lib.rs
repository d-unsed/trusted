#[macro_use] extern crate lazy_static;
#[macro_use] extern crate ruru;

extern crate hyper;

mod handler;
mod request;
mod request_processor;
mod response;
mod ruby;
mod server;

use ruby::Server;

#[no_mangle]
pub extern fn initialize_my_app() {
    Server::define_ruby_class();
}
