use std::io::{self, Read, stdin, stdout};
use termion::raw::IntoRawMode;
fn main() {
    let _stdoout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?}\r",b)
                } else {
                    println!("{:?} ({})\r",b,c)
                }
                if b == to_ctrl_byte('q') {
                    break;
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