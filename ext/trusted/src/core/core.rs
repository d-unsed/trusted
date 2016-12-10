use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::mpsc::{self, Receiver, Sender};

use ruru::{Class, Fixnum, Object, Proc, Thread};

use core::{Channel, ResponseFuture};
use request_processor::RequestProcessor;
use request::Request;
use response::Response;

use ruby::request::ProcessingPool;

pub struct Core {
    channel: Channel<ResponseFuture, Request>,
    ruby_handler: Proc,
    fake_stream: UnixStream,
    thread_pool_size: usize,
}

impl Core {
    pub fn new(sender: Sender<ResponseFuture>,
               receiver: Receiver<Request>,
               ruby_handler: Proc,
               fake_stream: UnixStream,
               thread_pool_size: usize) -> Self {

        Core {
            channel: Channel::new(sender, receiver),
            ruby_handler: ruby_handler,
            fake_stream: fake_stream,
            thread_pool_size: thread_pool_size,
        }
    }

    pub fn run(&mut self) {
        let args = vec![
            self.ruby_handler.to_any_object(),
            Fixnum::new(self.thread_pool_size as i64).to_any_object(),
        ];

        let processing_pool = Class::from_existing("Trusted")
            .get_nested_class("Request")
            .get_nested_class("ProcessingPool")
            .new_instance(args);

        // Can be casted unsafely, because processing pool is created directly from
        // ProcessingPool class
        let processing_pool = unsafe { processing_pool.to::<ProcessingPool>() };

        // Buffer for reading a byte from handlers when a fake socket is ready for reading
        let mut buffer = [0u8; 1];

        let fd = self.fake_stream.as_raw_fd();

        loop {
            println!("[rust] Waiting for request");

            // Handler will send one byte when new request is ready for processing
            // Thread::wait_fd() allows other Ruby threads to be scheduled while Core
            // is waiting for new requests
            Thread::wait_fd(fd);

            self.fake_stream.read(&mut buffer).unwrap();

            let request = self.channel.receiver().recv().unwrap();
            let (response_sender, response_receiver) = mpsc::channel::<Response>();

            self.channel.sender().send(response_receiver).unwrap();

            let observer = ::ruby::request::Observer::new(response_sender);

            RequestProcessor::new(request, &processing_pool).handle(observer);
        }
    }
}
