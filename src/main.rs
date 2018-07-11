use std::thread;

mod cli;


fn main() {
    let mut cli_interface = cli::Interface::new();

    let handle = thread::spawn(move || {
        cli_interface.listen();
    });

    handle.join().unwrap();
}
