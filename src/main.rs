use std::fs;

mod util;

use util::*;

fn main() {
    let content = match fs::read_to_string("assets/puzzle_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut hands = Vec::new();

    for line in content.lines() {
        let mut cards = Vec::new();

        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 2 {
            let number = parts[1].parse().unwrap();
            for card in parts[0].chars() {
                cards.push(match_card(card));
            }
            hands.push(Hand::new(cards, number));
        }
    }

    hands.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    // dbg!(&hands);

    let mut sum = 0;
    for (mut i, hand) in hands.iter().enumerate() {
        i += 1;
        sum += hand.bid as i32 * i as i32;
    }

    println!("{sum}");
}
