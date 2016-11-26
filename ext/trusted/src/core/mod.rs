mod channel;
mod core;
mod handler;
mod server;

use std::sync::mpsc::Receiver;

use response::Response;

pub use self::channel::Channel;
pub use self::core::Core;
pub use self::handler::Handler;
pub use self::server::Server;

pub type ResponseFuture = Receiver<Response>;
