use std::io::{self, Read, stdin, stdout};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

fn main() {
    let _stdoout = stdout().into_raw_mode().unwrap();
    for key in io::stdin().keys() {
        match key {
            Ok(key) => {
                match key {
                   Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8)
                        } else {
                            println!("{:?} ({})\r",c as u8,c)
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key),
                }
            }
            Err(err) => die(err),
        }
    }   
}

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte           
}

fn die(e: std::io::Error) {
    panic!("{}",e)
}