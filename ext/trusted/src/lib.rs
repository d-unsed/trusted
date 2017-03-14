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

use ruby::{Request, Response, Server};
use ruby::request::Observer;

#[no_mangle]
pub extern fn initialize_trusted_extension() {
    Observer::define_ruby_class();
    Request::define_ruby_class();
    Response::define_ruby_class();
    Server::define_ruby_class();
}
