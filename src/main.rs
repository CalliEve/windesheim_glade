#![allow(dead_code)]

mod map;
mod utils;

use utils::objects::Context;

fn main() {
    let i_bytes: Vec<u8> = std::fs::read("./instructions.txt").expect("no instructions file");
    let instructions = String::from_utf8(i_bytes).unwrap();

    let mut ctx = Context::new(instructions);

    ctx.execute();
}
