use std::sync::mpsc::Sender;

use ruru::{AnyObject, Class, NilClass, Object};

use ruby::Response;
use response::Response as RustResponse;

lazy_static! {
    static ref OBSERVER_CLASS: Class = {
        Class::from_existing("Trusted")
            .get_nested_class("Request")
            .get_nested_class("Observer")
    };
}

pub struct ObserverData {
    sender: Sender<RustResponse>,
}

wrappable_struct!(ObserverData, ObserverWrapper, OBSERVER_DATA_TYPE);

class!(Observer);

unsafe_methods!(
    Observer,
    itself,

    // This method is called when the `Future` finishes its execution
    // (when request is processed by Rack stack)
    //
    // Extract `Sender` which points to the corresponding Hyper handler and
    // send response to it
    fn update(_time: AnyObject, response: Response, _reason: AnyObject) -> NilClass {
        let data = itself.get_data(&*OBSERVER_DATA_TYPE);

        data.sender.send(response.into()).unwrap();

        NilClass::new()
    }
);

impl Observer {
    pub fn define_ruby_class() {
        let data_class = Class::from_existing("Data");

        Class::from_existing("Trusted")
            .get_nested_class("Request")
            .define_nested_class("Observer", Some(&data_class))
            .define(|itself| {
                itself.def("update", update);
            });
    }

    pub fn new(sender: Sender<RustResponse>) -> Self {
        let observer_data = ObserverData { sender: sender };
        (*OBSERVER_CLASS).wrap_data(observer_data, &*OBSERVER_DATA_TYPE)
    }
}
