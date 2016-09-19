use ruru::{Class, Fixnum, Hash, Object, RString};

use hyper::header::Headers;

class!(Response);

impl Response {
    pub fn new() -> Self {
        let response = Class::from_existing("Response").new_instance(vec![]);

        // We can use unsafe here, because response is created by our own code
        unsafe { response.to::<Self>() }
    }

    pub fn status(&self) -> i32 {
        // We can use unsafe here, because response is created by our own code
        let status = unsafe { self.send("status", vec![]).to::<Fixnum>().to_i64() };

        status as i32
    }

    pub fn headers(&self) -> Headers {
        let mut headers = Headers::new();

        // We can use unsafe here, because response is created by our own code
        let ruby_headers = unsafe { self.send("headers", vec![]).to::<Hash>() };

        ruby_headers.each(|name, value| {
            let name = unsafe { name.to::<RString>().to_string() };
            let value = unsafe { value.to::<RString>().to_string() };

            headers.set_raw(name.to_string(), vec![value.to_string().into_bytes()]);
        });

        headers
    }

    pub fn body(&self) -> String {
        // We can use unsafe here, because the hash is constructed by our own app
        unsafe { self.send("body", vec![]).to::<RString>().to_string_unchecked() }
    }
}
