extern crate termion;

use termion::color::{DetectColors, AnsiValue, Bg};
use termion::raw::IntoRawMode;
use std::io::stdout;

mod car;
mod support;

//2 -- green 1 -- red 0 -- black

fn main() {
    let car : Car = Car::new(0, 0);
    println!("f");
}