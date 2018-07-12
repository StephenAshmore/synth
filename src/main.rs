use std::thread;

extern crate terminal_size;

mod cli;


fn main() {
    let mut threads = vec![];

    let mut cli_interface = cli::Interface::new();
    let mut cli_display = cli::Display::new();

    threads.push(thread::spawn(move || {
        cli_interface.listen();
    }));

    threads.push(thread::spawn(move || {
        cli_display.listen();
    }));

    for t in threads {
        let _ = t.join();
    }
}
