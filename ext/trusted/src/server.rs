use std::sync::mpsc;
use std::thread;

use hyper::server::Server as HyperServer;
use hyperlocal::UnixSocketServer;
use ruru::{Proc, VM};

use config::{BindingType, Config};
use handler::Handler;
use request_processor::RequestProcessor;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config: config }
    }

    pub fn listen(self, ruby_handler: Proc) {
        println!("[rust] Start listening");

        let (request_sender, request_receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();

        let handler = Handler::new(request_sender, response_receiver);

        thread::spawn(move || {
            let handler_function = || -> () {
                match *self.config.binding_type() {
                    BindingType::Unix => {
                        UnixSocketServer::new(self.config.listen_on()).unwrap()
                            .handle(handler).unwrap();
                    },
                    BindingType::Tcp => {
                        HyperServer::http(self.config.listen_on()).unwrap()
                            .handle(handler).unwrap();
                    }
                };
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
