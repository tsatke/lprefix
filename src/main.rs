#![feature(stdin_forwarders)]
#![feature(box_syntax)]

use std::borrow::Borrow;
use std::env;
use std::io;

use rand::Rng;
use termion::color;
use termion::color::Color;

fn main() {
    let colors: Vec<Box<dyn Color>> = vec![
        box color::Yellow,
        box color::Red,
        box color::LightBlue,
        box color::Cyan,
        box color::Green,
        box color::Magenta,
    ];
    let i = rand::thread_rng().gen_range(0..colors.len());
    let c = colors[i].borrow();

    let prefix = env::args().nth(1).expect("require exactly 1 arg");
    io::stdin().lines().for_each(|line| {
        println!(
            "[{}{}{}] {}",
            color::Fg(c),
            prefix,
            color::Fg(color::Reset),
            line.unwrap(),
        );
    });
}
