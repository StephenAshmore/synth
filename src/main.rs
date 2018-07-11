use std::io::{self, Write, BufRead};


fn command_interface() -> String {
    print!("synth >> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    return stdin.lock().lines().next().unwrap().unwrap();
}

fn main() {
    let mut done = false;
    while !done {
        let input = command_interface();
        match &input as &str {
            "play" => println!("Play Synth"),
            "pause" => println!("Pause Synth"),
            "exit" => {
                println!("Goodbye...");
                done = true;
            },
            _ => println!("That command is not recognized."),
        }
    }
}
