use std::fs;

use util::{parse_direction, Direction, Location};

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

            let current_position = parts[0].trim();
            let directions_parts = parts[1]
                .trim_matches(|c| c == '(' || c == ')' || c == ' ')
                .split(", ")
                .collect::<Vec<_>>();

            if directions_parts.len() != 2 {
                continue;
            }

            let left_position = directions_parts[0];
            let right_position = directions_parts[1];

            let location = Location {
                position: current_position.to_string(),
                left: left_position.to_string(),
                right: right_position.to_string(),
            };

            locations.push(location);
        }
    }

    let mut current_position = "AAA";
    let end_position = "ZZZ";
    let mut count = 0;

    while &current_position != &end_position {
        for direction in &directions {
            count += 1;
            let find_location = locations
                .iter()
                .find(|x| x.position == current_position)
                .map(|x| {
                    if direction == &Direction::Left {
                        &x.left
                    } else {
                        &x.right
                    }
                });
            match find_location {
                Some(next_location) => {
                    current_position = next_location;
                }
                None => {
                    println!("Couldn't move {:?}: No location found", direction);
                    break;
                }
            }
        }
    }

    println!("position reached in {count} moves");

    // dbg!(&directions);
    // dbg!(&locations);
}
