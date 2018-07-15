use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

extern crate terminal_size;

mod cli;


fn main() {
    let mut threads = vec![];
    let mut user_finished = false;
    // can clone the sender: YAY
    let (sender, command_receiver) = mpsc::channel();
    let (display_command_distributor, display_distributor_receiver) = mpsc::channel();

    threads.push(thread::spawn(move || {
        let mut cli_interface = cli::Interface::new(sender.clone());
        cli_interface.listen();
    }));

    threads.push(thread::spawn(move || {
        let mut cli_display = cli::Display::new(display_distributor_receiver);
        cli_display.listen();
    }));

    while !user_finished {
        let cmd = command_receiver.recv().unwrap();
        display_command_distributor.send(cmd).unwrap();
    }

    for t in threads {
        let _ = t.join();
    }
}
