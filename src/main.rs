mod cli;

fn main() {
    let mut cli_interface = cli::Interface::new();
    cli_interface.listen();
}
