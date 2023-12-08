use std::fs::{self};

use util::get_farming_methods;

mod util;

fn main() {
    let content = match fs::read_to_string("assets/test_input.txt") {
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

    let farming_methods = get_farming_methods();
    let mut current_section = String::new();
    let mut methods = Vec::new();
    let mut max_range = 0;

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
                let end_range = source + range - 1;

                if end_range >= max_range {
                    max_range = end_range;
                }

                let mut found = false;

                if *seed >= source && *seed <= end_range {
                    found = true;
                }

                if found {
                    *seed += difference;
                    break;
                }
            }
        }
        methods.clear()
    }

    let awnser = seed_storage.iter().min().unwrap();

    println!("the lowest location number = {}", awnser);
}
