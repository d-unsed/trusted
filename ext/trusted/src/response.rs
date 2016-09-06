use std::convert::From;

use hyper::header::Headers;

use ruby::Response as RubyResponse;

pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: Headers,
}

impl From<RubyResponse> for Response {
    fn from(ruby_response: RubyResponse) -> Self {
        let headers = ruby_response.headers();

        Response {
            status: ruby_response.status() as u16,
            body: ruby_response.body(),
            headers: headers,
        }
    }
}
