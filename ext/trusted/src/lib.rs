#[macro_use] extern crate lazy_static;
#[macro_use] extern crate ruru;

extern crate hyper;
extern crate hyperlocal;

mod handler;
mod request;
mod request_processor;
mod response;
mod ruby;
mod server;

use ruby::{Request, Server};

#[no_mangle]
pub extern fn initialize_my_app() {
    Request::define_ruby_class();
    Server::define_ruby_class();
}
