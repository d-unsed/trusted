use std::io::Write;
use std::os::unix::net::UnixStream;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};

use hyper::server::{Handler as HyperHandler, Request as HyperRequest, Response as HyperResponse};
use hyper::status::StatusCode;

use core::{Channel, ResponseFuture};
use request::Request;

pub struct Handler {
    channel: Mutex<Channel<Request, ResponseFuture>>,
    stream: Mutex<UnixStream>,
}

impl Handler {
    pub fn new(sender: Sender<Request>,
               receiver: Receiver<ResponseFuture>,
               stream: UnixStream) -> Self {

        let channel = Channel::new(sender, receiver);

        Handler {
            channel: Mutex::new(channel),
            stream: Mutex::new(stream),
        }
    }

    fn response_future(&self, request: Request) -> ResponseFuture {
        let channel = self.channel.lock().unwrap();

        // Tell MRI to switch to main thread by sending data using file descriptor
        {
            let buffer = [0u8; 1];

            self.stream.lock().unwrap().write(&buffer).unwrap();
        }

        channel.sender().send(request).unwrap();

        // Receive a `ResponseFuture`
        channel.receiver().recv().unwrap()
    }
}

impl HyperHandler for Handler {
    fn handle(&self, hyper_request: HyperRequest, mut hyper_response: HyperResponse) {
        println!("[hyper] New request received");

        let request = hyper_request.into();
        let response_future = self.response_future(request);

        // Receive a response
        let response = response_future.recv().unwrap();

        {
            let headers = hyper_response.headers_mut();
            *headers = response.headers;
        }

        {
            let status = hyper_response.status_mut();
            *status = StatusCode::from_u16(response.status);
        }

        let mut res = hyper_response.start().unwrap();
        res.write_all(response.body.as_bytes()).unwrap();

        println!("[hyper] Response successfully sent to client");
        println!("[hyper] ------------------------------------");
    }
}
