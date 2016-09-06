use ruru::{Class, NilClass, RString, VM};
use ruru::traits::Object;

use server::Server as RustServer;

class!(Server);

methods!(
    Server,
    itself,

    fn initialize(addr: RString) -> Server {
        itself.instance_variable_set("@addr", addr);

        itself
    }

    fn listen() -> NilClass {
        let handler = VM::block_proc();
        let addr = itself.instance_variable_get("@addr").to::<RString>().to_string();

        RustServer::new(addr).listen(handler);

        NilClass::new()
    }
);

impl Server {
    pub fn define_ruby_class() {
        Class::new("RubyHttpServer").define(|itself| {
            itself.def("initialize", initialize);
            itself.def("listen", listen);
        });
    }
}
