use ruru::{Class, Fixnum, Hash, RString};
use ruru::traits::Object;

use hyper::header::Headers;

class!(Response);

impl Response {
    pub fn new() -> Self {
        let instance = Class::from_existing("Response").new_instance(vec![]);

        instance.to::<Self>()
    }

    pub fn status(&self) -> i32 {
        let status = self.send("status", vec![]).to::<Fixnum>().to_i64();

        status as i32
    }

    pub fn headers(&self) -> Headers {
        let mut headers = Headers::new();

        self.send("headers", vec![]).to::<Hash>().each(|name: RString, value: RString| {
            headers.set_raw(name.to_string(), vec![value.to_string().into_bytes()]);
        });

        headers
    }

    pub fn body(&self) -> String {
        self.send("body", vec![]).to::<RString>().to_string_unchecked()
    }
}
