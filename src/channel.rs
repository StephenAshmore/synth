use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub struct Message {
    type: string,
    value: string,
}

pub struct Channel {
    m_receiver: Receiver,
    m_sender: Sender,
}

impl Channel {
    pub fn new(s, r) -> Channel {
        Channel {
            m_receiver: r,
            m_sender: s,
        }
    }

    pub fn send(&mut self, message: Message) {
        m_sender.send(message).unwrap();
    }

    pub fn receive(&self) {
        return m_receiver.recv().unwrap();
    }
}