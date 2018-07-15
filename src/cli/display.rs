use std::io::{self, Write, BufRead};
use std::{thread, time};
use std::sync::mpsc::{Receiver};

use terminal_size::{Width, Height, terminal_size};

struct Cursor {
    x: u16,
    y: u16,
}

pub struct Display {
    m_receiver: Receiver<String>,
    m_user_finished: bool,
    m_terminal_height: u16,
    m_terminal_width: u16,
    m_cursor: Cursor,
}

impl Display {
    pub fn new(receiver: Receiver<String>) -> Display {
        let (w, h) = Display::get_terminal_size();

        Display {
            m_receiver: receiver,
            m_user_finished: false,
            m_terminal_height: h,
            m_terminal_width: w,
            m_cursor: Cursor {
                x: 0,
                y: 0,
            },
        }
    }

    pub fn listen(&mut self) {
        while !self.m_user_finished {
            let c = self.m_receiver.try_recv();
            let cmd = match c {
                Ok(cmd) => cmd,
                Err(error) => String::from(""),
            };
            // handle command here:
            match cmd.as_str() {
                "exit" => self.m_user_finished = true,
                _ => (),
            }
            self.display();
            thread::sleep(time::Duration::from_millis(100));
        }
    }
    
    fn display(&mut self) {
        self.update_size();
        self.clear();

        let mut buffer_pointer = 0;
        
        self.compute_cursor();

        for y in 0..self.m_terminal_height - 1 {
            if !self.border(y) {
                // write side border:
                for x in 0..self.m_terminal_width {
                    if x == 0 {
                        print!("|")
                    } else if x == self.m_terminal_width - 1 {
                        print!("|")
                    } else {
                        print!(" ")
                    }
                }
                // write non border stuff
                

                if y + 1 < self.m_terminal_height - 1 {
                    print!("\r\n");
                }
            }
        }

        // print!("{}[{}A", 27 as char, 2);
        // print!("{}[{}C", 27 as char, 2);
    }

    fn compute_cursor(&mut self) {
        self.m_cursor.y = self.m_terminal_height - 3;
        self.m_cursor.x = 10;
    }
    
    fn clear(&self) {
        print!("{}[{};{}f", 27 as char, 1, 1);
        io::stdout().flush();
    }

    fn border(&self, y: u16) -> bool {
        if y == 0 {
            for x in 0..self.m_terminal_width {
                if x == 0 || x == self.m_terminal_width - 1 {
                    print!("+");
                } else {
                    print!("-");
                }
            }
            return true;
        }
        return false;
    }

    fn update_size(&mut self) {
        let (w, h) = Display::get_terminal_size();
        self.m_terminal_width = w;
        self.m_terminal_height = h;
    }

    fn get_terminal_size() -> (u16, u16) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            return (w, h);
        } else {
            panic!("Unable to get terminal size");
        }
    }
}