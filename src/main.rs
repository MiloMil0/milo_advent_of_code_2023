use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut content = File::open("assets/test_input.txt").expect("couldn't find file");
    let mut buffer = String::new();

    content.read_to_string(&mut buffer)?;

    let lines = buffer.trim().split("\n").collect::<Vec<_>>();

    let sum = 0;

    dbg!("{}", lines);

    Ok(())
}
