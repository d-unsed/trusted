use std::convert::From;
use std::io::Read;

use hyper::header::Headers;
use hyper::server::{Request as HyperRequest};

pub struct Request {
    pub method: String,
    pub url: String,
    pub path_info: String,
    pub query_string: String,
    pub remote_addr: String,
    pub server_port: String,
    pub headers: Headers,
    pub body: String,
}

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

        Request {
            method: method,
            url: url,
            path_info: path_info,
            query_string: query_string,
            remote_addr: remote_addr,
            server_port: server_port,
            headers: request.headers,
            body: body,
        }
    }
}
