use std::io::Write;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};

use hyper::server::{Handler as HyperHandler, Request as HyperRequest, Response as HyperResponse};
use hyper::status::StatusCode;

use request::Request;
use response::Response;

pub struct Handler {
    sender: Mutex<Sender<Request>>,
    receiver: Mutex<Receiver<Response>>,
}

impl Handler {
    pub fn new(sender: Sender<Request>, receiver: Receiver<Response>) -> Self {
        Handler {
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        }
    }
}

impl HyperHandler for Handler {
    fn handle(&self, hyper_request: HyperRequest, mut hyper_response: HyperResponse) {
        println!("[hyper] New request received");

        self.sender.lock().unwrap().send(hyper_request.into()).unwrap();

        println!("[hyper] Sent to main thread");

        let response = self.receiver.lock().unwrap().recv().unwrap();

        println!("[hyper] Received response from main thread");

        {
            let headers = hyper_response.headers_mut();
            *headers = response.headers;
        }

        {
            let status = hyper_response.status_mut();
            *status = StatusCode::from_u16(response.status);
        }

        println!("[hyper] Starting to send response to client");

        let mut res = hyper_response.start().unwrap();
        res.write_all(response.body.as_bytes()).unwrap();

        println!("[hyper] Response successfully sent to client");
        println!("[hyper] ------------------------------------")
    }
}
