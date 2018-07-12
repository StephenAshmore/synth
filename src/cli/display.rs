use std::io::{self, Write, BufRead};
use std::{thread, time};

use terminal_size::{Width, Height, terminal_size};

pub struct Display {
    m_user_finished: bool,
    m_terminal_height: u16,
    m_terminal_width: u16,
}

impl Display {
    pub fn new() -> Display {
        let (w, h) = Display::get_terminal_size();

        Display {
            m_user_finished: false,
            m_terminal_height: h,
            m_terminal_width: w,
        }
    }

    pub fn listen(&mut self) {
        while !self.m_user_finished {
            self.display();
        }
    }
    
    fn display(&mut self) {
        self.clear();
        self.update_size();
        
        for y in 0..self.m_terminal_height {
            if !self.border(y) {
                // write non border stuff
                println!("");
            }
        }

        thread::sleep(time::Duration::from_millis(250));
    }
    
    fn clear(&self) {
        print!("{}[H{}[J", 27 as char, 27 as char);
        io::stdout().flush();
    }

    fn border(&self, y: u16) -> bool {
        if y == 0 || y == self.m_terminal_height - 1 {
            for x in 0..self.m_terminal_width {
                if x == 0 || x == self.m_terminal_width -1 {
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