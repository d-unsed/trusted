#[macro_use] extern crate lazy_static;
#[macro_use] extern crate ruru;

extern crate hyper;
extern crate hyperlocal;

mod config;
mod core;
mod request;
mod request_processor;
mod response;
mod ruby;

use ruby::{Request, Server};
use ruby::request::Observer;

#[no_mangle]
pub extern fn initialize_my_app() {
    Observer::define_ruby_class();
    Request::define_ruby_class();
    Server::define_ruby_class();
}
