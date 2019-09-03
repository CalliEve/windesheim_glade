#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod map;
mod objects;
mod regex;
mod weights;

use map::Glade;
use objects::Context;

fn main() {
    let mut instructions_file: Option<String> = None;
    let mut glade_file: Option<String> = None;
    let mut next_glade = false;
    let mut next_instructions = false;

    for arg in std::env::args() {
        if next_glade {
            next_glade = false;
            glade_file = Some(arg)
        } else if next_instructions {
            next_instructions = false;
            instructions_file = Some(arg)
        } else if arg == "-c" {
            next_instructions = true
        } else if arg == "-g" {
            next_glade = true
        }
    }

    let i_bytes: Vec<u8> =
        std::fs::read(instructions_file.unwrap_or_else(|| String::from("./instructions.txt")))
            .expect("no instructions file");
    let instructions = String::from_utf8(i_bytes).unwrap();
    let glade = Glade::parse(&glade_file.unwrap_or_else(|| String::from("./glade.csv")));

    let mut ctx = Context::new(instructions, glade);

    ctx.parse();
    ctx.execute();

    println!("FAILED");
    println!("points left: {}", 2020 - ctx.points);
}
