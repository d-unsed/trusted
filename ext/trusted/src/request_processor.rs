use ruru::{Object, Proc};

use request::Request;
use response::Response;
use ruby::{Request as RubyRequest, Response as RubyResponse};

pub struct RequestProcessor<'a> {
    request: Request,
    ruby_handler: &'a Proc,
}

impl<'a> RequestProcessor<'a> {
    pub fn new(request: Request, ruby_handler: &'a Proc) -> Self {
        RequestProcessor {
            request: request,
            ruby_handler: ruby_handler,
        }
    }

    pub fn handle(self) -> Response {
        let ruby_request = RubyRequest::from(self.request);
        let ruby_response = RubyResponse::new();

        self.ruby_handler.call(vec![ruby_request.to_any_object(), ruby_response.to_any_object()]);

        ruby_response.into()
    }
}
