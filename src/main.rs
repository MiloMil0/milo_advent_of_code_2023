use std::fs;

use util::{calculate_lcm, find_path_length, parse_direction, Location};

mod util;

fn main() {
    let content = match fs::read_to_string("assets/puzzle_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut is_direction = true;

    let mut directions = Vec::new();
    let mut locations = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            is_direction = false;
            continue;
        }

        if is_direction {
            directions.extend(parse_direction(line))
        } else {
            let parts = line.split(" = ").collect::<Vec<_>>();
            if parts.len() != 2 {
                continue;
            }

            let current_position = parts[0].chars().collect::<Vec<char>>();
            let directions_parts = parts[1]
                .trim_matches(|c| c == '(' || c == ')' || c == ' ')
                .split(", ")
                .map(|s| s.chars().collect())
                .collect::<Vec<Vec<char>>>();

            if directions_parts.len() != 2 {
                continue;
            }

            let left_position = directions_parts[0].clone();
            let right_position = directions_parts[1].clone();

            let location = Location {
                position: current_position,
                left: left_position,
                right: right_position,
            };

            locations.push(location);
        }
    }

    let mut path_lengths = Vec::new();

    locations
        .iter()
        .filter(|x| x.position.ends_with(&['A']))
        .for_each(|position| {
            if let Some(path_length) = find_path_length(position, &directions, &locations) {
                path_lengths.push(path_length);
            }
        });

    let result = calculate_lcm(&path_lengths);

    println!("the answer = {}", result);
}
