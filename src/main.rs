use std::fs;

use map::Map;

mod map;
mod point;
mod util;

fn main() {
    let content = match fs::read_to_string("assets/puzzle_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let starmap = Map::new(content);

    println!(
        "the shortest path between all stars = {} units of elf measurement",
        starmap.calculate_paths_between_galaxies()
    )
}
