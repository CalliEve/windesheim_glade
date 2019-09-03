#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod map;
mod objects;
mod regex;
mod weights;

use map::Glade;
use objects::Context;

fn main() {
    let i_bytes: Vec<u8> = std::fs::read("./instructions.txt").expect("no instructions file");
    let instructions = String::from_utf8(i_bytes).unwrap();
    let glade = Glade::parse("./glade.csv");

    let mut ctx = Context::new(instructions, glade);

    ctx.parse();
    ctx.execute();

    println!("FAILED");
    println!("points left: {}", 2020 - ctx.points);
}
