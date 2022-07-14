use std::io::{self, Read, stdin, stdout};
use termion::raw::IntoRawMode;
fn main() {
    let _stdoout = stdout().into_raw_mode().unwrap();

    for b in stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
    
        if c.is_control() {
            println!("{:?} \r",b)
        }
        else {
            println!("{:?} ({})", b, c)
        }
        if c == 'q' {
            break;
        }
    }
}
