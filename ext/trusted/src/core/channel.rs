use std::sync::mpsc::{Receiver, Sender};

pub struct Channel<S, R> {
    sender: Sender<S>,
    receiver: Receiver<R>,
}

impl<S, R> Channel<S, R> {
    pub fn new(sender: Sender<S>, receiver: Receiver<R>) -> Self {
        Channel {
            sender: sender,
            receiver: receiver,
        }
    }

    #[inline]
    pub fn sender(&self) -> &Sender<S> {
        &self.sender
    }

    #[inline]
    pub fn receiver(&self) -> &Receiver<R> {
        &self.receiver
    }
}
