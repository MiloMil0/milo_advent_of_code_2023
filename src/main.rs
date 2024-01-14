use std::collections::HashMap;

use crate::{parser::parse_content, spring::Record};

mod parser;
mod spring;
mod util;

fn main() {
    let content = parse_content("puzzle_input");
    let mut count = 0;

    let mut mem = HashMap::new();

    for line in content.lines() {
        let spring = Record::new(line);
        count += spring.possible_combinations(&mut mem);
    }

    println!("the sum of possible spring combinations = {count}");
}
