// use std::io::{self, Read};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn defualt() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?} \r");
        }
        println!("Sweet dreams!");
    }
    pub fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                println!("Event: {:?} \r", event);
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
