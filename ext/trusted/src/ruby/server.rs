use std::error::Error;

use ruru::{Class, NilClass, Object, VM};

use core::Server as RustServer;
use config::Config as RustConfig;
use ruby::Config;

class!(Server);

methods!(
    Server,
    itself,

    fn initialize(config: Config) -> Server {
        if let Err(ref error) = config {
            VM::raise(error.to_exception(), error.description());
        }

        itself.instance_variable_set("@config", config.unwrap());

        itself
    }

    fn listen() -> NilClass {
        let handler = VM::block_proc();

        // We can use unsafe here, because the type of @config is checked in the constructor
        let config = unsafe { itself.instance_variable_get("@config").to::<Config>() };

        RustServer::new(RustConfig::from(config)).listen(handler);

        NilClass::new()
    }
);

impl Server {
    pub fn define_ruby_class() {
        Class::from_existing("Trusted").define(|itself| {
            itself.define_nested_class("Server", None).define(|itself| {
                itself.def("initialize", initialize);
                itself.def("listen", listen);
            });
        });
    }
}
