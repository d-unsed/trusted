use request::Request;
use ruby::{Request as RubyRequest, Response as RubyResponse};
use ruby::request::{Observer, ProcessingPool};

pub struct RequestProcessor<'a> {
    request: Request,
    processing_pool: &'a ProcessingPool,
}

impl<'a> RequestProcessor<'a> {
    pub fn new(request: Request, processing_pool: &'a ProcessingPool) -> Self {
        RequestProcessor {
            request: request,
            processing_pool: processing_pool,
        }
    }

    pub fn handle(self, observer: Observer) {
        let ruby_request = RubyRequest::from(self.request);
        let ruby_response = RubyResponse::new();

        self.processing_pool.process(ruby_request, ruby_response, observer);
    }
}
