use std::sync::mpsc;
use std::thread;

use hyper::server::Server as HyperServer;
use ruru::{Proc, VM};

use handler::Handler;
use request_processor::RequestProcessor;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr: addr }
    }

    pub fn listen(self, ruby_handler: Proc) {
        println!("[rust] Start listening");

        let (request_sender, request_receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();

        let handler = Handler::new(request_sender, response_receiver);

        thread::spawn(move || {
            let handler_function = || -> () {
                HyperServer::http(self.addr.as_str()).unwrap().handle(handler).unwrap();
            };

            let unblock_function = || {
                ()
            };

            VM::thread_call_without_gvl(
                handler_function,
                Some(unblock_function)
            );
        });

        loop {
            println!("[rust] Trying to receive from sender");
            let request = request_receiver.recv().unwrap();

            println!("[rust] Sending to Ruby");
            let response = RequestProcessor::new(request, &ruby_handler).handle();

            println!("[rust] Received response from Ruby, sending back to Hyper");
            response_sender.send(response.into()).unwrap();
        }
    }
}
