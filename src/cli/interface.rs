use std::io::{self, BufRead};
use std::sync::mpsc::{Sender};
use std::sync::mpsc;

pub struct Interface {
    m_buffer: Vec<String>,
    m_sender: Sender<String>,
    m_user_finished: bool,
}

impl Interface {
    pub fn new(sender: Sender<String>) -> Interface {
        Interface {
            m_buffer: Vec::new(),
            m_sender: sender,
            m_user_finished: false,
        }
    }

    pub fn listen(&mut self) {
        while !self.m_user_finished {
            let input = self.get_input();
            self.append_to_buffer(input.trim());
            self.resolve_input(input.trim());
        }
    }

    fn resolve_input(&mut self, input: &str) {
        match &input as &str {
            "play" => println!("Play Synth"),
            "pause" => println!("Pause Synth"),
            "exit" => {
                println!("Goodbye...");
                self.m_user_finished = true;
            },
            _ => println!("That command is not recognized."),
        }
    }

    fn get_input(&mut self) -> String {
        let stdin = io::stdin();
        return stdin.lock().lines().next().unwrap().unwrap();
    }

    fn append_to_buffer(&mut self, new_input: &str) {
        self.m_buffer.push(new_input.to_string());
    }
}