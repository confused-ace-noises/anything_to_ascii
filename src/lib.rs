pub mod chars;
pub use chars::*;

pub mod ascii_img;

pub mod dimensions;

pub mod cli;

#[test]
fn thing() {
    for x in 0..=15 {
        println!("{}", x*17)
    }
}