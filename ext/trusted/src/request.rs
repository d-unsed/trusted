use std::convert::From;
use std::io::Read;

use hyper::header::Headers;
use hyper::server::{Request as HyperRequest};

pub struct Request {
    method: String,
    url: String,
    path_info: String,
    query_string: String,
    remote_addr: String,
    server_port: String,
    headers: Headers,
    body: String,
}

impl Request {
    pub fn new(method: String,
               url: String,
               path_info: String,
               query_string: String,
               remote_addr: String,
               server_port: String,
               headers: Headers,
               body: String)
               -> Self {
        Request {
            method: method,
            url: url,
            path_info: path_info,
            query_string: query_string,
            remote_addr: remote_addr,
            server_port: server_port,
            headers: headers,
            body: body,
        }
    }

    #[inline]
    pub fn method(&self) -> &str {
        &self.method
    }

    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }

    #[inline]
    pub fn path_info(&self) -> &str {
        &self.path_info
    }

    #[inline]
    pub fn query_string(&self) -> &str {
        &self.query_string
    }

    #[inline]
    pub fn remote_addr(&self) -> &str {
        &self.remote_addr
    }

    #[inline]
    pub fn server_port(&self) -> &str {
        &self.server_port
    }

    #[inline]
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    #[inline]
    pub fn body(&self) -> &str {
        &self.body
    }
}

wrappable_struct!(Request, RequestWrapper, REQUEST_DATA_TYPE);

impl<'a, 'b> From<HyperRequest<'a, 'b>> for Request {
    fn from(mut request: HyperRequest) -> Self {
        let mut body = String::new();

        request.read_to_string(&mut body).unwrap();

        let method = request.method.to_string();
        let url = request.uri.to_string();
        let parsed_url = url.clone();
        let parsed_url = parsed_url.split('?').collect::<Vec<&str>>();

        let path_info = parsed_url.get(0).map(|s| *s).unwrap_or("").to_string();
        let query_string = parsed_url.get(1).map(|s| *s).unwrap_or("").to_string();

        let remote_addr = request.remote_addr.ip().to_string();
        let server_port = request.remote_addr.port().to_string();

        Request::new(method,
                     url,
                     path_info,
                     query_string,
                     remote_addr,
                     server_port,
                     request.headers,
                     body)
    }
}
