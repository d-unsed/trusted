use std::convert::From;

use ruru::{Class, Hash, RString};
use ruru::traits::Object;

use request::Request as RustRequest;

class!(Request);

impl From<RustRequest> for Request {
    fn from(request: RustRequest) -> Self {
        let mut headers = Hash::new();

        for header in request.headers.iter() {
            let field = header.name().to_string().replace("-", "_").to_uppercase();
            let value = header.value_string();

            let field = format!("HTTP_{}", field);

            headers.store(RString::new(&field), RString::new(&value));
        }

        Class::from_existing("Request").new_instance(
            vec![
                RString::new(&request.method).to_any_object(),
                RString::new(&request.url).to_any_object(),
                RString::new(&request.path_info).to_any_object(),
                RString::new(&request.query_string).to_any_object(),
                RString::new(&request.remote_addr).to_any_object(),
                RString::new(&request.server_port).to_any_object(),
                headers.to_any_object(),
                RString::new(&request.body).to_any_object(),
            ]
        ).to::<Self>()
    }
}
