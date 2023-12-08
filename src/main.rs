use std::fs::{self};

use util::get_farming_methods;

mod util;

fn main() {
    let content = match fs::read_to_string("assets/puzzle_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut seed_storage = Vec::new();

    for line in content.lines() {
        if line.starts_with("seeds:") {
            seed_storage = line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split_whitespace()
                .filter_map(|string| string.parse().ok())
                .collect::<Vec<i64>>();
        }
    }

    let seed_pairs: Vec<(i64, i64)> = seed_storage
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
        .collect();

    let farming_methods = get_farming_methods();
    let mut current_section = String::new();
    let mut methods = Vec::new();
    let mut awnser = i64::MAX;

    for items in seed_pairs.iter() {
        seed_storage.clear();
        for id in items.0..=items.0 + items.1 {
            seed_storage.push(id);
        }
        for method in farming_methods.iter() {
            for line in content.lines() {
                if line.is_empty() {
                    current_section.clear();
                }
                if line.starts_with(method) {
                    current_section = line.split_whitespace().next().unwrap_or("").to_string();
                }
                if !current_section.is_empty() {
                    let values = line
                        .split_whitespace()
                        .filter_map(|string| string.parse().ok())
                        .collect::<Vec<i64>>();

                    methods.push(values);
                }
            }

            for seed in seed_storage.iter_mut() {
                for method in methods.iter() {
                    if method.is_empty() {
                        continue;
                    }
                    let destination = method[0];
                    let source = method[1];
                    let range = method[2];

                    let difference = destination - source;
                    let end_range = source + range;

                    let mut found = false;

                    if *seed >= source && *seed < end_range {
                        found = true;
                    }

                    if found {
                        *seed += difference;
                        break;
                    }
                }
            }
            methods.clear();
        }

        let temp = seed_storage.iter().min().unwrap();
        println!("{temp}");
        if *temp <= awnser {
            awnser = *temp;
        }
    }
    println!("the lowest location number = {}", awnser);
}
