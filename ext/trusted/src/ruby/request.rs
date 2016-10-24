use std::convert::From;

use ruru::{Class, Hash, Object, RString};

use request::{REQUEST_DATA_TYPE, Request as RustRequest};

lazy_static! {
    static ref REQUEST_CLASS: Class = {
        Class::from_existing("Trusted").get_nested_class("Request")
    };
}

class!(Request);

methods!(
    Request,
    itself,

    fn method() -> RString {
        RString::new(itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).method())
    }

    fn uri() -> RString {
        RString::new(itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).url())
    }

    fn path_info() -> RString {
        RString::new(itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).path_info())
    }

    fn query_string() -> RString {
        RString::new(itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).query_string())
    }

    fn remote_addr() -> RString {
        RString::new(itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).remote_addr())
    }

    fn server_port() -> RString {
        RString::new(itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).server_port())
    }

    fn headers() -> Hash {
        let mut headers = Hash::new();

        for header in itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).headers().iter() {
            let field = header.name().to_string().replace("-", "_").to_uppercase();
            let value = header.value_string();

            let field = format!("HTTP_{}", field);

            headers.store(RString::new(&field), RString::new(&value));
        }

        headers
    }

    fn body() -> RString {
        RString::new(itself.get_data::<RustRequest>(&*REQUEST_DATA_TYPE).body())
    }
);

impl Request {
    pub fn define_ruby_class() {
        Class::from_existing("Trusted").define_nested_class("Request", None).define(|itself| {
            itself.def("method", method);
            itself.def("uri", uri);
            itself.def("path_info", path_info);
            itself.def("query_string", query_string);
            itself.def("remote_addr", remote_addr);
            itself.def("server_port", server_port);
            itself.def("headers", headers);
            itself.def("body", body);
        });
    }
}

impl From<RustRequest> for Request {
    fn from(request: RustRequest) -> Self {
        (*REQUEST_CLASS).wrap_data(request, &*REQUEST_DATA_TYPE)
    }
}
