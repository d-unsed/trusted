use std::error::Error;

use ruru::{Class, NilClass, RString, Object, VM};

use server::Server as RustServer;

class!(Server);

methods!(
    Server,
    itself,

    fn initialize(addr: RString) -> Server {
        if let Err(ref error) = addr {
            VM::raise(error.to_exception(), error.description());
        }

        itself.instance_variable_set("@addr", addr.unwrap());

        itself
    }

    fn listen() -> NilClass {
        let handler = VM::block_proc();

        // We can use unsafe here, because the type of addr is checked in the constructor
        let addr = unsafe { itself.instance_variable_get("@addr").to::<RString>().to_string() };

        RustServer::new(addr).listen(handler);

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
