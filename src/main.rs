use std::fs;

use crate::map::Map;

mod map;
mod point;
mod util;

fn main() {
    let content = match fs::read_to_string("assets/test_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut map = Map::new(content);

    let result = map.try_move();
    println!("the longest path is {}", result);
}
