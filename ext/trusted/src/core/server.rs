use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread;

use hyper::server::Server as HyperServer;
use hyperlocal::UnixSocketServer;
use ruru::{Proc, VM};

use config::{BindingType, Config};
use core::{Core, Handler, ResponseFuture};
use request::Request;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config: config }
    }

    pub fn listen(self, ruby_handler: Proc) {
        println!("[rust] Start listening");

        let (handler_stream, core_stream) = UnixStream::pair().unwrap();

        let (request_sender, request_receiver) = mpsc::channel::<Request>();
        let (response_future_sender, response_future_receiver) = mpsc::channel::<ResponseFuture>();

        let handler = Handler::new(request_sender, response_future_receiver, handler_stream);

        let mut core = Core::new(response_future_sender,
                                 request_receiver,
                                 ruby_handler,
                                 core_stream,
                                 self.config.rack_thread_pool_size());

        thread::spawn(move || {
            let handler_function = || -> () {
                println!("[hyper] GVL released for server thread");

                let address = self.config.listen_on();
                let thread_pool_size = self.config.native_thread_pool_size();

                println!("[hyper] Spawning {} native thread(s)", thread_pool_size);
                println!("[hyper] Listening on {}", address);

                match *self.config.binding_type() {
                    BindingType::Unix => {
                        UnixSocketServer::new(address).unwrap()
                            .handle_threads(handler, thread_pool_size).unwrap();
                    },
                    BindingType::Tcp => {
                        HyperServer::http(address).unwrap()
                            .handle_threads(handler, thread_pool_size).unwrap();
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

        core.run();
    }
}
