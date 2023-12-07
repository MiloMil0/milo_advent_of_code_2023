use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    game_numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
}

impl Card {
    fn new(game_numbers: Vec<i32>, winning_numbers: Vec<i32>) -> Self {
        Card {
            game_numbers,
            winning_numbers,
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut content = File::open("assets/puzzle_input.txt").expect("couldn't find file");
    let mut buffer = String::new();

    content.read_to_string(&mut buffer)?;

    let mut cards = buffer.trim().split("\n").collect::<Vec<_>>();

    let mut deck = HashMap::new();

    for (idx, card) in cards.iter_mut().enumerate() {
        let key = idx + 1;
        *card = card
            .splitn(2, ":")
            .nth(1)
            .map(|string| string.trim())
            .unwrap_or_default();

        if let Some((left, right)) = card.split_once("|") {
            let winning_numbers = left
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect::<Vec<i32>>();
            let game_numbers = right
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect::<Vec<i32>>();

            let game_card = Card::new(game_numbers, winning_numbers);

            deck.entry(key).or_insert(Vec::new()).push(game_card);
        }
    }

    let mut card_order = deck.keys().cloned().collect::<Vec<_>>();
    card_order.sort();

    let mut card_counter = HashMap::new();
    let mut points = 1;

    for cards in card_order {
        card_counter.entry(cards).or_insert(1);
        if let Some(card) = deck.get(&cards) {
            card.iter().for_each(|card| {
                card.game_numbers.iter().for_each(|game_num| {
                    card.winning_numbers.iter().for_each(|win_num| {
                        if game_num == win_num {
                            points += 1;
                            for _ in 0..*card_counter.get(&cards).unwrap() {
                                *card_counter.entry(points).or_insert(1) += 1;
                            }
                        }
                    })
                });
                points = cards + 1;
            })
        }
    }

    println!("the awnser = {}", card_counter.values().sum::<i32>());

    Ok(())
}
